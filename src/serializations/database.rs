// Libs
use serde::{Deserialize, Serialize};

// Structs
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Trainer {
    pub id: u128,
    pub name: String,
    pub pokedex: Vec<DatabasePokemon>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DatabasePokemon {
    pub id: u16,
    pub is_shiny: bool,
}
