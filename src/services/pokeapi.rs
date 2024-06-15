// Libs
use once_cell::sync::Lazy;
use std::{process::exit, sync::Arc};
use tokio::sync::RwLock;
use tracing::{error, info, Instrument};

use crate::di::cache::{Cache, RedisCache};
use crate::di::http_client::{HTTPClient, ReqwestHTTPClient};
use crate::errors::PokeAPIResult;
use crate::serializations::cache::CachedPokemon;
use crate::serializations::pokeapi::{
    PokeAPIPokemon, PokeAPIPokemonEvolutionChain, PokeAPIPokemonSpecies, PokeAPIPokemonSpeciesCount,
};

// Data
pub static POKEAPI_SERVICE: Lazy<Arc<PokeAPI<RedisCache, ReqwestHTTPClient>>> = Lazy::new(|| {
    let redis_instance = RedisCache::new();
    if let Err(e) = redis_instance {
        error!("Error creating Redis instance: {:?}", e);
        exit(1);
    };

    Arc::new(PokeAPI::new(
        RwLock::new(redis_instance.unwrap()),
        ReqwestHTTPClient::new(),
    ))
});

// PokeAPI
/**
The PokeAPI service is a service that interacts with the PokeAPI to get information about pokemons.

It uses a cache to store the information of the pokemons to avoid making requests to the PokeAPI every time.

It needs to be used in a Arc.

## Type Parameters
- `T`: The type of the HTTPClient to use.
- `U`: The type of the Cache to use.
*/
pub struct PokeAPI<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    cache: RwLock<T>,
    http_client: U,
    pokeapi_url: String,
}

impl<T, U> PokeAPI<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    /**
    A method to create a new instance of the PokeAPI service.

    ## Parameters:
    - `cache`: The cache to use for the service. It should implement the Cache trait.
    - `http_client`: The HTTPClient to use for the service. It should implement the HTTPClient trait.
    */
    pub fn new(cache: RwLock<T>, http_client: U) -> Self {
        Self {
            cache,
            http_client,
            pokeapi_url: std::env::var("POKEAPI_URL").unwrap(),
        }
    }

    /**
    A method to connect the cache to the service.
    */
    pub async fn connect_to_cache(&self) -> PokeAPIResult<()> {
        info!("[POKEAPI] Connecting the cache...");
        let mut cache = self.cache.write().await;
        cache.connect().await?;
        info!("[POKEAPI] The cache has been connected.");
        Ok(())
    }

    /**
    A method to find some pokemon by their identifiers.

    ## Parameters:
    - `identifiers`: The identifiers of the pokemons to find.
    */
    pub async fn find_poke(&self, identifier: &str) -> PokeAPIResult<CachedPokemon> {
        info!(
            "[POKEAPI] Checking if the pokemon#{} is in the cache...",
            identifier
        );
        let cache = self.cache.read().await;

        let poke = cache.get_key(identifier).await?;
        if let Some(poke) = poke {
            info!("[POKEAPI] The pokemon#{} is in the cache.", identifier);
            return Ok(serde_json::from_str(&poke).unwrap());
        }

        info!(
            "[POKEAPI] The pokemon#{} is not in the cache. Retrieving from the PokeAPI...",
            identifier
        );
        let cached_poke = self
            .create_cached_poke(identifier)
            .in_current_span()
            .await?;

        info!("[POKEAPI] Inserting the pokemon in the cache...");
        for key in vec![cached_poke.id.to_string(), cached_poke.name.to_lowercase()] {
            cache
                .insert_key(&key, &serde_json::to_string(&cached_poke).unwrap())
                .await?;
        }

        info!("[POKEAPI] The pokemon has been inserted in the cache.");
        Ok(cached_poke)
    }

    /**
    A method to get the amount of pokemons in the PokeAPI.
    */
    pub async fn get_pokemons_count(&self) -> PokeAPIResult<u16> {
        info!("[PokeAPI] Getting the amount of pokemons in the PokeAPI...");
        info!("[PokeAPI] Checking if the amount of pokemons is in the cache...");
        let cache = self.cache.read().await;

        let poke_count = cache.get_key("pokemons_count").await?;
        if let Some(poke_count) = poke_count {
            info!("[PokeAPI] The amount of pokemons is in the cache.");
            return Ok(poke_count.parse().unwrap());
        }

        info!(
            "[PokeAPI] The amount of pokemons is not in the cache. Retrieving from the PokeAPI..."
        );
        let url = format!("{}/pokemon-species/", self.pokeapi_url);
        let poke_count = self
            .http_client
            .access::<PokeAPIPokemonSpeciesCount>("GET", &url)
            .await?
            .count;

        info!("[PokeAPI] Inserting the amount of pokemons in the cache...");
        cache
            .insert_key("pokemons_count", &poke_count.to_string())
            .await?;

        info!("[PokeAPI] The amount of pokemons has been inserted in the cache.");
        Ok(poke_count)
    }

    /**
    A method to create a cached pokemon from a pokemon, a species, and an evolution chain.

    ## Parameters
    - `identifier`: The identifier of the pokemon.
    */
    async fn create_cached_poke(&self, identifier: &str) -> PokeAPIResult<CachedPokemon> {
        // Get the pokemon, the species, and the evolution chain.
        let poke = self.get_poke(identifier);
        let poke_species = self.get_poke_species(identifier);

        // Wait for the results.
        let (poke, poke_species) = tokio::try_join!(poke, poke_species)?;
        let poke_chain_id: Vec<&str> = poke_species.evolution_chain.url.split('/').rev().collect();
        let poke_evolution_chain = self.get_poke_evolution_chain(poke_chain_id[1]).await?;

        Ok(CachedPokemon::merge(
            poke,
            poke_species,
            poke_evolution_chain,
        ))
    }

    /**
    A method to get a pokemon by its identifier.

    ## Parameters
    - `identifier`: The identifier of the pokemon to get.
    */
    async fn get_poke(&self, identifier: &str) -> PokeAPIResult<PokeAPIPokemon> {
        let url = format!("{}/pokemon/{}", self.pokeapi_url, identifier);
        Ok(self.http_client.access("GET", &url).await?)
    }

    /**
    A method to get a species by its identifier.

    ## Parameters
    - `identifier`: The identifier of the species to get.
    */
    async fn get_poke_species(&self, identifier: &str) -> PokeAPIResult<PokeAPIPokemonSpecies> {
        let url = format!("{}/pokemon-species/{}", self.pokeapi_url, identifier);
        Ok(self
            .http_client
            .access::<PokeAPIPokemonSpecies>("GET", &url)
            .await?)
    }

    /**
    A method to get a evolution chain by its identifier.
    */
    async fn get_poke_evolution_chain(
        &self,
        identifier: &str,
    ) -> PokeAPIResult<PokeAPIPokemonEvolutionChain> {
        let url = format!("{}/evolution-chain/{}", self.pokeapi_url, identifier);
        Ok(self
            .http_client
            .access::<PokeAPIPokemonEvolutionChain>("GET", &url)
            .await?)
    }
}
