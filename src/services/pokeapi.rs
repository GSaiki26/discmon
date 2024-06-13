// Libs
use reqwest::get;
use serde::de::DeserializeOwned;

use crate::serializations::pokemon::{
    PokeAPIPokemon, PokeAPIPokemonEvolutionChain, PokeAPIPokemonSpecies,
};

// Structs
/**
A method to get a pokemon by its identifier.

## Parameters
- `identifier`: The identifier of the pokemon to get.
*/
pub async fn get_poke(identifier: &str) -> reqwest::Result<PokeAPIPokemon> {
    let pokeapi_url = std::env::var("POKEAPI_URL").unwrap();
    let url = format!("{}/pokemon/{}", pokeapi_url, identifier);
    get_resource(&url).await
}

/**
A method to get a species by its identifier.

## Parameters
- `identifier`: The identifier of the species to get.
*/
pub async fn get_poke_species(identifier: &str) -> reqwest::Result<PokeAPIPokemonSpecies> {
    let pokeapi_url = std::env::var("POKEAPI_URL").unwrap();
    let url = format!("{}/pokemon-species/{}", pokeapi_url, identifier);
    get_resource(&url).await
}

/**
A method to get a evolution chain by its identifier.
*/
pub async fn get_poke_evolution_chain(
    identifier: &u16,
) -> reqwest::Result<PokeAPIPokemonEvolutionChain> {
    let pokeapi_url = std::env::var("POKEAPI_URL").unwrap();
    let url = format!("{}/evolution-chain/{}", pokeapi_url, identifier);
    get_resource(&url).await
}

/**
A method to get some resource by its url.
*/
async fn get_resource<T>(url: &str) -> reqwest::Result<T>
where
    T: DeserializeOwned,
{
    let res = get(url).await?;
    res.json().await
}
