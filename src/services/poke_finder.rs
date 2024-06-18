// Libs
use once_cell::sync::Lazy;
use std::{process::exit, sync::Arc};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::di::{Cache, RedisCache};
use crate::di::{HTTPClient, ReqwestHTTPClient};
use crate::errors::PokeFinderResult;
use crate::serializations::cache::CachedPokemon;
use crate::serializations::pokeapi::{
    PokeAPIPokemon, PokeAPIPokemonEvolutionChain, PokeAPIPokemonSpecies, PokeAPIPokemonSpeciesCount,
};
use crate::utils::EnvManager;

// Data
pub static POKEFINDER_SERVICE: Lazy<Arc<PokeFinder<RedisCache, ReqwestHTTPClient>>> =
    Lazy::new(|| {
        let redis_instance = RedisCache::new();
        if let Err(e) = redis_instance {
            error!("Error creating Redis instance: {:?}", e);
            exit(1);
        };

        Arc::new(PokeFinder::new(
            RwLock::new(redis_instance.unwrap()),
            ReqwestHTTPClient::new(),
        ))
    });

// PokeFinder
/**
The PokeFinder service is a service that interacts with the PokeFinder to get information about pokemons.

It uses a cache to store the information of the pokemons to avoid making requests to the PokeFinder every time.

It needs to be used in a Arc.

## Type Parameters
- `T`: The type of the HTTPClient to use.
- `U`: The type of the Cache to use.
*/
pub struct PokeFinder<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    cache: RwLock<T>,
    http_client: U,
}

impl<T, U> PokeFinder<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    /**
    A method to create a new instance of the PokeFinder service.

    ## Parameters:
    - `cache`: The cache to use for the service. It should implement the Cache trait.
    - `http_client`: The HTTPClient to use for the service. It should implement the HTTPClient trait.
    */
    pub fn new(cache: RwLock<T>, http_client: U) -> Self {
        Self { cache, http_client }
    }

    /**
    A method to connect the cache to the service.
    */
    pub async fn connect_to_cache(&self) -> PokeFinderResult<()> {
        info!("Connecting the cache...");
        let mut cache = self.cache.write().await;
        cache.connect().await?;
        info!("The cache has been connected.");
        Ok(())
    }

    /**
    A method to find some pokemon by their identifiers.

    ## Parameters:
    - `identifiers`: The identifiers of the pokemons to find.
    */
    pub async fn find_poke(&self, identifier: &str) -> PokeFinderResult<CachedPokemon> {
        info!("Checking if the pokemon#{} is in the cache...", identifier);
        let cache = self.cache.read().await;

        let poke = cache.get_key(identifier).await?;
        if let Some(poke) = poke {
            info!("The pokemon#{} is in the cache.", identifier);
            return Ok(serde_json::from_str(&poke).unwrap());
        }

        info!(
            "The pokemon#{} is not in the cache. Retrieving from the PokeAPI...",
            identifier
        );
        let cached_poke = self.create_cached_poke(identifier).await?;

        info!("Inserting the pokemon in the cache...");
        for key in [cached_poke.id.to_string(), cached_poke.name.to_lowercase()] {
            cache
                .insert_key(&key, &serde_json::to_string(&cached_poke).unwrap())
                .await?;
        }

        info!("The pokemon has been inserted in the cache.");
        Ok(cached_poke)
    }

    /**
    A method to get the amount of pokemons in the PokeFinder.
    */
    pub async fn get_poke_count(&self) -> PokeFinderResult<u16> {
        info!("Getting the amount of pokemons in the PokeAPI...");
        info!("Checking if the amount of pokemons is in the cache...");
        let cache = self.cache.read().await;

        let poke_count = cache.get_key("pokemons_count").await?;
        if let Some(poke_count) = poke_count {
            info!("The amount of pokemons is in the cache.");
            return Ok(poke_count.parse().unwrap());
        }

        info!("The amount of pokemons is not in the cache. Retrieving from the PokeAPI...");
        let url = format!(
            "{}/pokemon-species/",
            EnvManager::get_var::<String>("POKEAPI_URL")
        );
        let poke_count = self
            .http_client
            .access::<PokeAPIPokemonSpeciesCount>("GET", &url)
            .await?
            .count;

        info!("Inserting the amount of pokemons in the cache...");
        cache
            .insert_key("pokemons_count", &poke_count.to_string())
            .await?;

        info!("The amount of pokemons has been inserted in the cache.");
        Ok(poke_count)
    }

    /**
    A method to create a cached pokemon from a pokemon, a species, and an evolution chain.

    ## Parameters
    - `identifier`: The identifier of the pokemon.
    */
    async fn create_cached_poke(&self, identifier: &str) -> PokeFinderResult<CachedPokemon> {
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
    async fn get_poke(&self, identifier: &str) -> PokeFinderResult<PokeAPIPokemon> {
        let url = format!(
            "{}/pokemon/{}",
            EnvManager::get_var::<String>("POKEAPI_URL"),
            identifier
        );
        Ok(self.http_client.access("GET", &url).await?)
    }

    /**
    A method to get a species by its identifier.

    ## Parameters
    - `identifier`: The identifier of the species to get.
    */
    async fn get_poke_species(&self, identifier: &str) -> PokeFinderResult<PokeAPIPokemonSpecies> {
        let url = format!(
            "{}/pokemon-species/{}",
            EnvManager::get_var::<String>("POKEAPI_URL"),
            identifier
        );
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
    ) -> PokeFinderResult<PokeAPIPokemonEvolutionChain> {
        let url = format!(
            "{}/evolution-chain/{}",
            EnvManager::get_var::<String>("POKEAPI_URL"),
            identifier
        );
        Ok(self
            .http_client
            .access::<PokeAPIPokemonEvolutionChain>("GET", &url)
            .await?)
    }
}
