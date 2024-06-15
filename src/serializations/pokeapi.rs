// Libs
use serde::{Deserialize, Serialize};

// Pokemon
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemon {
    pub height: u16,
    pub id: u16,
    pub is_default: bool,
    pub name: String,
    pub species: PokeAPIResource,
    pub sprites: PokeAPISprites,
    pub stats: Vec<PokeAPIStat>,
    pub types: Vec<PokeAPIType>,
    pub weight: u16,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIType {
    pub slot: u8,
    pub r#type: PokeAPIResource,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIStat {
    pub base_stat: u16,
    pub effort: u16,
    pub stat: PokeAPIResource,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISprites {
    pub front_default: String,
    pub front_female: Option<()>,
    pub front_shiny: String,
    pub front_shiny_female: Option<()>,
    pub other: PokeAPISpritesOther,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISpritesOther {
    #[serde(rename = "official-artwork")]
    pub official_artwork: PokeAPISpritesOtherOfficialArtwork,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISpritesOtherOfficialArtwork {
    pub front_default: String,
    pub front_shiny: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIResource {
    pub name: String,
    pub url: String,
}

// Pokemon Species
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonSpecies {
    pub evolution_chain: PokeAPIEvolutionchain,
    pub evolves_from_species: Option<PokeAPIResource>,
    pub has_gender_differences: bool,
    pub hatch_counter: u16,
    pub id: u16,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub name: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIEvolutionchain {
    pub url: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonSpeciesCount {
    pub count: u16,
}

// Pokemon Evolution Chain
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonEvolutionChain {
    pub chain: PokeAPIChain,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIChain {
    pub evolves_to: Vec<PokeAPIChain>,
    pub species: PokeAPIResource,
}
