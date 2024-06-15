// Libs

use serenity::all::{CreateEmbed, CreateMessage, Mentionable, User};

use crate::serializations::cache::CachedPokemon;

// Functions
/**
A method to get a message for when a wild pokemon has appeared.

## Parameters:
- `poke`: The pokemon that has appeared.
*/
pub fn get_msg_wild_pokemon_appeared(poke: &CachedPokemon) -> CreateMessage {
    let embed = CreateEmbed::new()
        .title(format!("A wild {} has appeared!", poke.name.to_uppercase()))
        .image(&poke.sprites.other.official_artwork.front_default)
        // .field("Pokemon", format!("#{}", poke.id), true)
        .description("*Be quick to catch it or it will run away!*");
    CreateMessage::new().embed(embed)
}

/**
A method to get a message for when a wild pokemon has fled.

## Parameters:
- `poke`: The pokemon that has fled.
*/
pub fn get_msg_wild_pokemon_fled(poke: &CachedPokemon) -> CreateMessage {
    let embed = CreateEmbed::new()
        .title(format!("The wild {} has fled!", poke.name.to_uppercase()))
        .description("You were too slow to catch it!");
    CreateMessage::new().embed(embed)
}

/**
A method to get a message for when a wild pokemon has been caught.

## Parameters:
- `poke`: The pokemon that has been caught.
*/
pub fn get_msg_wild_pokemon_caught(poke: &CachedPokemon, user: User) -> CreateMessage {
    let embed = CreateEmbed::new()
        .title(format!("You caught a {}!", poke.name.to_uppercase()))
        .description(format!(
            "Congratulations {} you caught a new pokémon!",
            user.mention()
        ));
    CreateMessage::new().embed(embed)
}
