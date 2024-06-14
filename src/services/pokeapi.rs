// Libs
use crate::{
    di::{
        cache::{Cache, CacheError, RedisCache},
        http_client::{HTTPClient, HTTPClientError, ReqwestHTTPClient},
    },
    serializations::pokemon::{
        CachedPokemon, PokeAPIPokemon, PokeAPIPokemonEvolutionChain, PokeAPIPokemonSpecies,
        PokeAPIPokemonSpeciesCount,
    },
};
use once_cell::sync::Lazy;
use std::{process::exit, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info};

// Type
pub type PokeAPIResult<T> = Result<T, PokeAPIError>;

// Data
pub const POKEAPI_URL: Lazy<String> = Lazy::new(|| std::env::var("POKEAPI_URL").unwrap());
pub const POKEAPI_SERVICE: Lazy<Arc<PokeAPI<RedisCache, ReqwestHTTPClient>>> = Lazy::new(|| {
    let reqwest_instance = ReqwestHTTPClient::new();
    let redis_instance = RedisCache::new();
    if let Err(e) = redis_instance {
        error!("Error creating Redis instance: {:?}", e);
        exit(1);
    };

    Arc::new(PokeAPI::new(
        Arc::new(Mutex::new(redis_instance.unwrap())),
        Arc::new(RwLock::new(reqwest_instance)),
    ))
});

// PokeAPIError
#[derive(Debug)]
pub enum PokeAPIError {
    HTTPClientError(HTTPClientError),
    CacheError(CacheError),
    Other(String),
}

impl From<HTTPClientError> for PokeAPIError {
    fn from(error: HTTPClientError) -> Self {
        PokeAPIError::HTTPClientError(error)
    }
}

impl From<CacheError> for PokeAPIError {
    fn from(error: CacheError) -> Self {
        PokeAPIError::CacheError(error)
    }
}

impl From<String> for PokeAPIError {
    fn from(error: String) -> Self {
        PokeAPIError::Other(error)
    }
}

impl From<&str> for PokeAPIError {
    fn from(error: &str) -> Self {
        PokeAPIError::Other(error.to_string())
    }
}

// PokeAPI
/**
The PokeAPI service is a service that interacts with the PokeAPI to get information about pokemons.
It uses a cache to store the information of the pokemons to avoid making requests to the PokeAPI every time.

## Type Parameters
- `T`: The type of the HTTPClient to use.
- `U`: The type of the Cache to use.
*/
pub struct PokeAPI<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    cache: Arc<Mutex<T>>,
    http_client: Arc<RwLock<U>>,
    pokeapi_url: String,
}

impl<T, U> PokeAPI<T, U>
where
    T: Cache,
    U: HTTPClient,
{
    /**
    A method to create a new instance of the PokeAPI service.
    */
    pub fn new(cache: Arc<Mutex<T>>, http_client: Arc<RwLock<U>>) -> Self {
        Self {
            cache,
            http_client,
            pokeapi_url: POKEAPI_URL.to_string(),
        }
    }

    /**
    A method to find some pokemon by their identifiers.

    ## Parameters:
    - `identifiers`: The identifiers of the pokemons to find.
    */
    pub async fn find_poke(&self, identifier: &str) -> Result<CachedPokemon, PokeAPIError> {
        // Check if the pokemon is in the cache.
        let cache = self.cache.lock().await;
        info!(
            "[POKEAPI] Checking if the pokemon#[{}] is in the cache...",
            identifier
        );
        let poke = cache.get_key(identifier).await?;
        if let Some(poke) = poke {
            info!("[POKEAPI] The pokemon is in the cache.");
            return Ok(serde_json::from_str(&poke).unwrap());
        }

        info!("[POKEAPI] The pokemon is not in the cache. Retrieving from the PokeAPI...");
        let cached_poke = self.create_cached_poke(identifier).await?;

        info!("[POKEAPI] Inserting the pokemon in the cache...");
        cache
            .insert_key(
                &cached_poke.name,
                &serde_json::to_string(&cached_poke).unwrap(),
            )
            .await?;
        cache
            .insert_key(
                &cached_poke.id.to_string(),
                &serde_json::to_string(&cached_poke).unwrap(),
            )
            .await?;

        info!("[POKEAPI] The pokemon has been inserted in the cache.");
        Ok(cached_poke)
    }

    /**
    A method to get the amount of pokemons in the PokeAPI.
    */
    pub async fn get_pokemons_count(&self) -> Result<u16, PokeAPIError> {
        info!("[PokeAPI] Getting the amount of pokemons in the PokeAPI...");
        info!("[PokeAPI] Checking if the amount of pokemons is in the cache...");
        let a = self.cache.lock().await.get_key("pokemons_count").await?;
        if let Some(a) = a {
            info!("[PokeAPI] The amount of pokemons is in the cache.");
            return Ok(a.parse().unwrap());
        }

        info!(
            "[PokeAPI] The amount of pokemons is not in the cache. Retrieving from the PokeAPI..."
        );
        let url = format!("{}/pokemon-species/", self.pokeapi_url);
        let count: PokeAPIPokemonSpeciesCount =
            self.http_client.read().await.access("GET", &url).await?;

        info!("[PokeAPI] Inserting the amount of pokemons in the cache...");
        self.cache
            .lock()
            .await
            .insert_key("pokemons_count", &count.count.to_string())
            .await?;

        Ok(count.count)
    }

    /**
    A method to create a cached pokemon from a pokemon, a species, and an evolution chain.

    ## Parameters
    - `identifier`: The identifier of the pokemon.
    */
    async fn create_cached_poke(&self, identifier: &str) -> Result<CachedPokemon, PokeAPIError> {
        // Get the pokemon, the species, and the evolution chain.
        let poke = self.get_poke(identifier).await?;
        let poke_species = self.get_poke_species(identifier).await?;
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
    async fn get_poke(&self, identifier: &str) -> Result<PokeAPIPokemon, PokeAPIError> {
        let url = format!("{}/pokemon/{}", self.pokeapi_url, identifier);
        Ok(self.http_client.read().await.access("GET", &url).await?)
    }

    /**
    A method to get a species by its identifier.

    ## Parameters
    - `identifier`: The identifier of the species to get.
    */
    async fn get_poke_species(
        &self,
        identifier: &str,
    ) -> Result<PokeAPIPokemonSpecies, PokeAPIError> {
        let url = format!("{}/pokemon-species/{}", self.pokeapi_url, identifier);
        Ok(self
            .http_client
            .read()
            .await
            .access::<PokeAPIPokemonSpecies>("GET", &url)
            .await?)
    }

    /**
    A method to get a evolution chain by its identifier.
    */
    async fn get_poke_evolution_chain(
        &self,
        identifier: &str,
    ) -> Result<PokeAPIPokemonEvolutionChain, PokeAPIError> {
        let url = format!("{}/evolution-chain/{}", self.pokeapi_url, identifier);
        Ok(self
            .http_client
            .read()
            .await
            .access::<PokeAPIPokemonEvolutionChain>("GET", &url)
            .await?)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::di::{
//         cache::tests::MockCacheMock,
//         http_client::{HTTPClientMock, ReqwestHTTPClient},
//     };

//     #[tokio::test]
//     pub async fn test_find_pokes_in_cache() -> Result<()> {
//         // Data
//         std::env::set_var("POKEAPI_URL", "some_url");
//         __mock_MockCache

//         let cache = Arc::new(Mutex::new(CacheMock::default()));
//         let http_client = Arc::new(RwLock::new(HTTPClientMock::default()));
//         let pokeapi = PokeAPI::new(cache.clone(), http_client.clone());

//         // Mocks
//         let poke = CachedPokemon::default();
//         cache.lock().await.get_key_return = Some(serde_json::to_string(&poke).unwrap());

//         // Tests
//         let pokes = pokeapi.find_pokes(vec!["1", "2"]).await?;
//         let cache = cache.lock().await;
//         let cache_calls = cache.get_key_calls.lock().await;
//         assert_eq!(pokes.len(), 2);
//         assert_eq!(pokes[0].id, poke.id);
//         assert_eq!(pokes[1].id, poke.id);
//         assert_eq!(cache_calls.len(), 2);
//         assert_eq!(cache.insert_key_calls.lock().await.len(), 0);
//         assert_eq!(cache_calls[0], String::from("1"));
//         assert_eq!(cache_calls[1], String::from("2"));
//         assert_eq!(http_client.read().await.access_calls.lock().await.len(), 0);

//         Ok(())
//     }

//     #[tokio::test]
//     pub async fn test_find_pokes_no_cache() -> Result<()> {
//         // Data
//         std::env::set_var("POKEAPI_URL", "some_url");

//         let cache = Arc::new(Mutex::new(CacheMock::default()));
//         let http_client = Arc::new(RwLock::new(
//             crate::di::http_client::HTTPClientMock::default(),
//         ));
//         let pokeapi = PokeAPI::new(cache.clone(), http_client.clone());

//         // Mocks
//         let poke = CachedPokemon::default();
//         cache.lock().await.get_key_return = None;
//         http_client.write().await.access_return = serde_json::to_string(&poke).unwrap();

//         // Tests
//         let pokes = pokeapi.find_pokes(vec!["1", "2"]).await?;
//         let cache = cache.lock().await;
//         assert_eq!(pokes.len(), 2);
//         assert_eq!(cache.get_key_calls.lock().await.len(), 2);
//         assert_eq!(cache.insert_key_calls.lock().await.len(), 0);
//         assert_eq!(cache.get_key_calls.lock().await[0], String::from("1"));
//         assert_eq!(cache.get_key_calls.lock().await[1], String::from("2"));

//         Ok(())
//     }
// }
