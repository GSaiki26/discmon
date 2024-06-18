// Libs
use serde::{Deserialize, Serialize};

// Pokemon
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIType {
    pub slot: u8,
    pub r#type: PokeAPIResource,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIStat {
    pub base_stat: u16,
    pub effort: u16,
    pub stat: PokeAPIResource,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISprites {
    pub front_default: String,
    pub front_female: Option<String>,
    pub front_shiny: String,
    pub front_shiny_female: Option<String>,
    pub other: PokeAPISpritesOther,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISpritesOther {
    #[serde(rename = "official-artwork")]
    pub official_artwork: PokeAPISpritesOtherOfficialArtwork,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPISpritesOtherOfficialArtwork {
    pub front_default: String,
    pub front_shiny: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIResource {
    pub name: String,
    pub url: String,
}

// Pokemon Species
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonSpecies {
    pub evolution_chain: PokeAPIEvolutionchain,
    pub evolves_from_species: Option<PokeAPIResource>,
    pub has_gender_differences: bool,
    pub id: u16,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIEvolutionchain {
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonSpeciesCount {
    pub count: u16,
}

// Pokemon Evolution Chain
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIPokemonEvolutionChain {
    pub chain: PokeAPIChain,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PokeAPIChain {
    pub evolves_to: Vec<PokeAPIChain>,
    pub species: PokeAPIResource,
}
