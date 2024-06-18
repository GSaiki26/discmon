// Libs
use serenity::{
    all::{
        parse_emoji, CommandInteraction, Context, CreateButton, CreateEmbed, CreateEmbedAuthor,
        CreateInteractionResponse, CreateInteractionResponseMessage, Message,
    },
    futures::StreamExt,
};
use std::{sync::Arc, time::Duration};
use surrealdb::sql::Thing;
use tracing::{debug, info};

use crate::{
    errors::HandlerResult,
    messages::{get_msg_pokedex_ack, get_msg_pokedex_content, get_msg_pokedex_empty},
    models::{DBPoke, DBTrainer},
    serializations::cache::CachedPokemon,
    services::POKEFINDER_SERVICE,
    utils::EnvManager,
};

// Functions
/**
A method to get the pokemons owned by the trainer.

## Parameters:
- `trainer_id`: The trainer's ID.

## Returns:
- A tuple with the pokemons owned by the trainer and the pokemons species.
*/
async fn get_trainer_pokes(trainer_id: &Thing) -> HandlerResult<(Vec<DBPoke>, Vec<DBPoke>)> {
    let mut trainer_pokes = DBPoke::find_by_trainer_id(trainer_id).await?;
    trainer_pokes.sort_by(|a, b| a.poke_id.cmp(&b.poke_id));
    let mut trainer_species = trainer_pokes.clone();
    trainer_species.dedup_by(|a, b| a.poke_id == b.poke_id);

    Ok((trainer_pokes, trainer_species))
}

/**
A method to extract a `page` from a list of pokémons.
This page is created based on the environment variable `BOT_POKEDEX_POKES_PER_PAGE`.

If the requested page doesn't exist, an empty array'll be returned.

## Parameters:
- `total_pages`: The total number of pages.
- `current_page`: The requested page.
- `pokelist`: A list of pokémons.

## Returns:
- An array of pokémons.
*/
fn get_pokedex_page(
    total_pages: impl Into<u16>,
    current_page: impl Into<u16>,
    pokelist: &Vec<DBPoke>,
) -> Vec<DBPoke> {
    // Convert the parameters into the correct types.
    let total_pages: u16 = total_pages.into();
    let current_page: u16 = current_page.into();

    // Get the total number of pokemons per page and select the current page.
    let poke_per_page: u16 = EnvManager::get_var("BOT_POKEDEX_POKES_PER_PAGE");

    info!("Getting the pokedex page {current_page}/{total_pages}...");
    if current_page > total_pages {
        info!("Page not found. Returning an empty array.");
        return vec![];
    }

    // Get the pokemons for the current page.
    let start_poke = (current_page - 1) * poke_per_page;
    let end_poke = start_poke + poke_per_page;

    let mut pokedex_page = Vec::new();
    for i in start_poke..end_poke {
        if i as usize >= pokelist.len() {
            break;
        }

        pokedex_page.push(pokelist[i as usize].clone());
    }

    info!("The pokedex page {current_page}/{total_pages} was created.");
    pokedex_page
}

/**
A method to transform a pokemon list into a pokedex description.

The poke_cache'll be updated if the pokémon list is not found.

## Parameters:
- `pokelist`: A list of pokémons.
- `poke_cache`: A list of all cached pokémons.

## Returns:
- An array of strings with each description.

*/
async fn describe_trainer_pokes(
    trainer_pokes: &Vec<DBPoke>,
    poke_cache: &mut Vec<CachedPokemon>,
) -> HandlerResult<Vec<String>> {
    let pokefinder_svc = POKEFINDER_SERVICE.clone();
    let mut descriptions: Vec<String> = Vec::new();

    for (index, trainer_poke) in trainer_pokes.iter().enumerate() {
        // Get the pokemon's attributes.
        let total_caught = trainer_pokes
            .iter()
            .filter(|p| p.poke_id == trainer_poke.poke_id)
            .count();
        let total_shiny_caught = trainer_pokes
            .iter()
            .filter(|p| p.poke_id == trainer_poke.poke_id && p.is_shiny)
            .count();
        let last_captured = &trainer_pokes
            .iter()
            .filter(|p| p.poke_id == trainer_poke.poke_id)
            .max_by(|a, b| a.created_at.cmp(&b.created_at))
            .unwrap()
            .created_at;

        // Check if the pokémon is in the cache.
        let cached_poke = match poke_cache.iter().find(|p| p.id == trainer_poke.poke_id) {
            Some(cached_poke) => cached_poke.to_owned(),
            None => {
                let cached_poke = pokefinder_svc
                    .find_poke(&trainer_poke.poke_id.to_string())
                    .await?;

                poke_cache.push(cached_poke.clone());
                cached_poke
            }
        };

        descriptions.push(format!(
    "{}) **[#{}] {}**\n* {} **Total caught**: {}\n* ✨ **Total shiny caught**: {}\n* 📅 **Last captured**: {}",
    index + 1,
    trainer_poke.poke_id,
    cached_poke.name.to_uppercase(),
    parse_emoji(EnvManager::get_var::<String>("BOT_POKEBALL_EMOJI_ID")).unwrap(),
    total_caught,
    total_shiny_caught,
    last_captured.format("%Y/%m/%d %H:%M")))
    }

    Ok(descriptions)
}

// Pokedex Handler
/**
A struct to handle the pokedex of the bot.
*/
pub struct PokedexHandler {
    ctx: Arc<Context>,
    command: CommandInteraction,
}
impl PokedexHandler {
    /**
    A method to create a new PokedexHandler.
    */
    pub fn new(ctx: Arc<Context>, command: CommandInteraction) -> Self {
        Self { ctx, command }
    }

    /**
    A method to handle the pokedex command.
    */
    pub async fn handle(&self) -> HandlerResult<()> {
        info!("Handling the pokedex command...");

        // Get the trainer's pokemons.
        self.send_ack_message().await?;
        let trainer = self.get_command_owner().await?;
        let total_pokes = POKEFINDER_SERVICE.clone().get_poke_count().await?;
        let (trainer_pokes, trainer_species) = get_trainer_pokes(&trainer.id).await?;

        // Get the total number of pages and pokemons per page.
        let pokes_per_page = EnvManager::get_var("BOT_POKEDEX_POKES_PER_PAGE");
        let total_pages = trainer_species.len() as u16 / pokes_per_page + 1;

        // Check if the trainer has any pokemons.
        if trainer_pokes.len() == 0 {
            return self.send_empty_pokedex_msg().await;
        }

        // Get the total number of pages and pokemons per page.

        // Define the pokedex pages and cache.
        let mut poke_cache = Vec::new();

        // Mount the pokedex message.
        info!("Mounting the pokedex message...");
        let pokedex_page_pokes = get_pokedex_page(total_pages, 1_u16, &trainer_pokes);
        let pokedex_page = describe_trainer_pokes(&pokedex_page_pokes, &mut poke_cache).await?;
        let pokedex_msg_content = get_msg_pokedex_content(
            &self.command.user,
            trainer_pokes.len() as u16,
            trainer_species.len() as u16,
            total_pokes,
            1_u16,
            total_pages,
            pokes_per_page,
            &pokedex_page.join("\n\n"),
        );

        // Send the pokedex message.
        info!("Sending the pokedex message...");
        let pokedex_msg = self
            .command
            .channel_id
            .send_message(&self.ctx, pokedex_msg_content)
            .await?;

        // Handle the interactions.
        self.handle_pokedex_interactions(
            pokedex_msg,
            total_pages,
            pokes_per_page,
            total_pokes,
            trainer_pokes,
            trainer_species,
            poke_cache,
        )
        .await
    }

    /**
    A method to send the ack message.
    */
    async fn send_ack_message(&self) -> HandlerResult<()> {
        info!("Sending the ack message...");
        let message = CreateInteractionResponse::Message(get_msg_pokedex_ack());
        self.command.create_response(&self.ctx, message).await?;
        Ok(())
    }

    /**
    A method to find the pokedex owner.

    ## Returns:
    - The trainer that owns the pokedex.
    */
    pub async fn get_command_owner(&self) -> HandlerResult<DBTrainer> {
        let user_id = self.command.user.id.to_string();
        let guild_id = self.command.guild_id.ok_or("Guild ID not found.")?;
        Ok(DBTrainer::find_by_discord_id(&user_id, &guild_id.to_string()).await?)
    }

    /**
    A method to send the empty pokedex message.
    */
    pub async fn send_empty_pokedex_msg(&self) -> HandlerResult<()> {
        info!("No pokemons found. Sending a message...");
        self.command
            .channel_id
            .send_message(&self.ctx, get_msg_pokedex_empty(&self.command.user.name))
            .await?;

        return Ok(());
    }

    /**
    A method to handle the pokedex's interactions.
    */
    pub async fn handle_pokedex_interactions(
        &self,
        pokedex_msg: Message,
        total_pages: u16,
        total_pokes_per_page: u16,
        total_pokes: u16,
        trainer_pokes: Vec<DBPoke>,
        trainer_species: Vec<DBPoke>,
        mut poke_cache: Vec<CachedPokemon>,
    ) -> HandlerResult<()> {
        info!("Waiting for pokedex interactions...");

        let mut current_page = 1;
        let mut interactions = pokedex_msg
            .await_component_interaction(&self.ctx.shard)
            .timeout(Duration::from_secs(EnvManager::get_var(
                "BOT_POKEDEX_TIMEOUT_SECS",
            )))
            .stream();

        while let Some(interaction) = interactions.next().await {
            // Check if the interaction is from the trainer.
            if interaction.user != self.command.user {
                info!("Interaction received from other user. Ignoring...");
                continue;
            }

            // Check the interaction type.
            info!("Interaction received.");
            match interaction.data.custom_id.as_str() {
                "pokedex_previous" => {
                    if current_page > 1 {
                        current_page -= 1;
                    }
                }
                "pokedex_next" => {
                    if current_page != total_pages {
                        current_page += 1
                    }
                }
                _ => {
                    info!("Invalid interaction. Ignoring...");
                    debug!("Interaction: {}", interaction.data.custom_id.as_str());
                    continue;
                }
            }

            // Update the pokedex message.
            let pokedex_page_pokes = get_pokedex_page(total_pages, current_page, &trainer_pokes);
            let pokedex_page = describe_trainer_pokes(&pokedex_page_pokes, &mut poke_cache).await?;

            let title = format!("{}'s Pokédex 📕", self.command.user.name);
            let author = CreateEmbedAuthor::from(self.command.user.clone());
            let embed = CreateEmbed::new()
                .title(title)
                .author(author)
                .field(
                    "Total caught pokémons",
                    trainer_pokes.len().to_string(),
                    true,
                )
                .field(
                    "Registered pokémons",
                    format!("{}/{}", trainer_species.len(), total_pokes),
                    true,
                )
                .field(
                    format!("Current page ({} per page)", total_pokes_per_page),
                    format!("{}/{}", current_page, total_pages),
                    true,
                )
                .description(pokedex_page.join("\n\n"));

            let pokedex_previous = CreateButton::new("pokedex_previous").label("Previous");
            let pokedex_next = CreateButton::new("pokedex_next").label("Next");
            let message = CreateInteractionResponseMessage::new()
                .add_embed(embed)
                .button(pokedex_previous)
                .button(pokedex_next);

            interaction
                .create_response(&self.ctx, CreateInteractionResponse::UpdateMessage(message))
                .await?;
        }

        Ok(())
    }
}
