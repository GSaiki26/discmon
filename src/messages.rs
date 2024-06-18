// Libs
use serenity::all::{
    CreateEmbed, CreateEmbedAuthor, CreateInteractionResponseMessage, CreateMessage, Mention, User,
};

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
A method to get a message to ack the /pokedex command.
*/
pub fn get_msg_pokedex_ack() -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new().content("Retrieving the pokedex...")
}

/**
A method to get the empty pokedex message.
*/
pub fn get_msg_pokedex_empty(username: &str) -> CreateMessage {
    CreateMessage::new().embed(
        CreateEmbed::new()
            .title(format!("{}'s Pokédex 📕", username))
            .description("No pokémons found. Go catch some! 🎣"),
    )
}

/**
A method to create the message for the pokedex content.

## Parameters:
- `user`: The user that called the command.
- `total_caught`: The total number of pokemons caught by the user.
- `registered_pokes`: The total number of pokemons registered by the user. it must be in the "{}/{}" format.
- `page_index`: The current page index. it must be in the "{}/{}" format.
- `pokes_per_page`: The total number of pokemons per page.
- `pokedex_description`: The pokedex description in markdown.
*/
pub fn get_embed_pokedex_content(
    user: &User,
    total_caught: u16,
    registered_pokes: &str,
    page_index: &str,
    pokes_per_page: u16,
    pokedex_description: &str,
) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("{}'s Pokédex 📕", user.name))
        .author(CreateEmbedAuthor::from(user.clone()))
        .field("Total caught pokémons", total_caught.to_string(), true)
        .field("Registered pokémons", registered_pokes, true)
        .field(
            format!("Current page ({} per page)", pokes_per_page),
            page_index,
            true,
        )
        .description(pokedex_description)
}

/**
A method to get a message when a dev command has been called.
*/
#[cfg(feature = "dev_commands")]
pub fn get_dev_error_msg_poke_spawn() -> CreateMessage {
    let embed = CreateEmbed::new()
.title("❌ Error spawning a random pokemon!")
.description("An error occurred while spawning a random pokemon. Please check the documentation and follows the proper usage.\n\n```\n!dev spawn_random [poke_flee_time_secs] [is_poke_shiny]\n```")
.field("poke_flee_time_secs", "Number", true)
.field("is_poke_shiny", "Boolean", true);
    CreateMessage::new().embed(embed)
}
