// Libs
use serenity::all::{CreateEmbed, CreateMessage, Mention};

use crate::serializations::cache::CachedPokemon;

// Functions
/**
A method to get a message when a wild pokemon has appeared.

## Parameters:
- `is_shiny`: A boolean to check if the pokemon is shiny.
- `poke`: The pokemon that has appeared.
*/
pub fn get_msg_wild_pokemon_appeared(is_shiny: bool, poke: &CachedPokemon) -> CreateMessage {
    let title = match is_shiny {
        true => format!("A wild shiny {} ✨ has appeared!", poke.name.to_uppercase()),
        false => format!("A wild {} has appeared!", poke.name.to_uppercase()),
    };
    let sprite_url: &str = match is_shiny {
        true => &poke.sprites.other.official_artwork.front_shiny,
        false => &poke.sprites.other.official_artwork.front_default,
    };

    let embed = CreateEmbed::new()
        .title(title)
        .image(sprite_url)
        .description("*Be quick to catch it or it will run away!*");
    CreateMessage::new().embed(embed)
}

/**
A method to get a message when a wild pokemon has fled.

## Parameters:
- `is_shiny`: A boolean to check if the pokemon is shiny.
- `poke_name`: The name of the pokemon that has fled. The function'll uppercase it.
*/
pub fn get_msg_wild_pokemon_fled(is_shiny: bool, poke_name: &str) -> CreateMessage {
    let title = match is_shiny {
        true => format!("The wild shiny {} ✨ has fled!", poke_name.to_uppercase()),
        false => format!("The wild {} has fled!", poke_name.to_uppercase()),
    };
    let embed = CreateEmbed::new()
        .title(title)
        .description("You were too slow to catch it!");
    CreateMessage::new().embed(embed)
}

/**
A method to get a message when a wild pokemon has been caught.

## Parameters:
- `is_shiny`: A boolean to check if the pokemon is shiny.
- `poke_name`: The name of the pokemon that was caught. The function'll uppercase it.
- `mention`: The user that caught the pokemon.
*/
pub fn get_msg_wild_pokemon_caught(
    is_shiny: bool,
    poke_name: &str,
    mention: Mention,
) -> CreateMessage {
    let title = match is_shiny {
        true => format!("You caught a shiny {} ✨!", poke_name.to_uppercase()),
        false => format!("You caught a {}!", poke_name.to_uppercase()),
    };
    let embed = CreateEmbed::new().title(title).description(format!(
        "Congratulations {} you caught a new pokémon!",
        mention
    ));
    CreateMessage::new().embed(embed)
}

/**
A method to get a message when a dev command has been called.
*/
pub fn get_dev_error_msg_poke_spawn() -> CreateMessage {
    let embed = CreateEmbed::new()
.title("❌ Error spawning a random pokemon!")
.description("An error occurred while spawning a random pokemon. Please check the documentation and follows the proper usage.\n\n```\n!dev spawn_random [poke_flee_time_secs] [is_poke_shiny]\n```")
.field("poke_flee_time_secs", "Number", true)
.field("is_poke_shiny", "Boolean", true);
    CreateMessage::new().embed(embed)
}
