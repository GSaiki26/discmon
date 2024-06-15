// Libs
use serde::{Deserialize, Serialize};

use super::pokeapi::{
    PokeAPIChain, PokeAPIPokemon, PokeAPIPokemonEvolutionChain, PokeAPIPokemonSpecies,
    PokeAPIResource, PokeAPISprites, PokeAPIStat, PokeAPIType,
};

// Cached Pokemon
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CachedPokemon {
    pub height: u16,
    pub id: u16,
    pub is_default: bool,
    pub name: String,
    pub sprites: PokeAPISprites,
    pub stats: Vec<PokeAPIStat>,
    pub types: Vec<PokeAPIType>,
    pub weight: u16,
    pub evolves_from_species: Option<PokeAPIResource>,
    pub has_gender_differences: bool,
    pub hatch_counter: u16,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub evolves_to: Vec<PokeAPIChain>,
}

impl CachedPokemon {
    /**
    Merge the Pokemon, Species and Evolution Chain into a single struct.

    ## Parameters:
    - `pokemon`: PokeAPIPokemon
    - `species`: PokeAPIPokemonSpecies
    - `evolution_chain`: PokeAPIPokemonEvolutionChain
    */
    pub fn merge(
        pokemon: PokeAPIPokemon,
        species: PokeAPIPokemonSpecies,
        evolution_chain: PokeAPIPokemonEvolutionChain,
    ) -> Self {
        CachedPokemon {
            height: pokemon.height,
            id: pokemon.id,
            is_default: pokemon.is_default,
            name: pokemon.name,
            sprites: pokemon.sprites,
            stats: pokemon.stats,
            types: pokemon.types,
            weight: pokemon.weight,
            evolves_from_species: species.evolves_from_species,
            has_gender_differences: species.has_gender_differences,
            hatch_counter: species.hatch_counter,
            is_baby: species.is_baby,
            is_legendary: species.is_legendary,
            is_mythical: species.is_mythical,
            evolves_to: evolution_chain.chain.evolves_to,
        }
    }
}
