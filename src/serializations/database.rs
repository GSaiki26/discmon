// Libs
use serde::{Deserialize, Serialize};

// Structs
#[derive(Debug, Deserialize, Serialize)]
pub struct Trainer<Id, DateTime> {
    pub _id: Id,
    pub discord_id: u128,
    pub discord_guild_id: u128,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabasePokemon<Id, DateTime> {
    pub _id: Id,
    pub trainer_id: Id,
    pub poke_id: u16,
    pub is_shiny: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
