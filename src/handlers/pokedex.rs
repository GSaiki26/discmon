// Libs
use serenity::{
    all::{
        parse_emoji, CommandInteraction, Context, CreateButton, CreateEmbed,
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, Message, User,
    },
    futures::StreamExt,
};
use std::{sync::Arc, time::Duration};
use tracing::{debug, info};

use crate::{
    errors::HandlerResult,
    messages::{get_embed_pokedex_content, get_msg_pokedex_ack, get_msg_pokedex_empty},
    models::{DBPoke, DBTrainer},
    serializations::cache::CachedPokemon,
    services::POKEFINDER_SERVICE,
    utils::{trainer::get_trainer_owned_pokes, EnvManager},
};

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
    pokelist: &[DBPoke],
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
    trainer_pokes: &[DBPoke],
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

/**
A method to mount the pokemon's embed message.

## Parameters:
- `user`: The user that requested the pokedex.
- `pokedex_info`: The pokedex information.
*/
async fn mount_pokedex_content_embed(
    user: &User,
    pokedex_info: &mut PokedexInfo,
) -> HandlerResult<CreateEmbed> {
    info!("Mounting the pokedex message...");
    let pokedex_page_pokes = get_pokedex_page(
        pokedex_info.total_pages,
        pokedex_info.current_page,
        &pokedex_info.trainer_pokes,
    );
    let pokedex_page =
        describe_trainer_pokes(&pokedex_page_pokes, &mut pokedex_info.poke_cache).await?;
    let embed = get_embed_pokedex_content(
        user,
        pokedex_info.trainer_pokes.len() as u16,
        &format!(
            "{}/{}",
            pokedex_info.trainer_species.len(),
            pokedex_info.total_pokes
        ),
        &format!("1/{}", pokedex_info.total_pages),
        pokedex_info.pokes_per_page,
        &pokedex_page.join("\n\n"),
    );

    info!("The pokedex message was mounted.");
    Ok(embed)
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

        // Get the pokedex information.
        let mut pokedex_info = PokedexInfo::new(self.command.clone()).await?;

        // Check if the trainer has any pokemons.
        if pokedex_info.trainer_pokes.is_empty() {
            return self.send_empty_pokedex_msg().await;
        }

        // Mount the pokedex message.
        let embed = mount_pokedex_content_embed(&self.command.user, &mut pokedex_info).await?;
        let pokedex_msg_content = CreateMessage::new()
            .add_embed(embed)
            .button(CreateButton::new("pokedex_previous").label("Previous"))
            .button(CreateButton::new("pokedex_next").label("Next"));

        // Send the pokedex message.
        info!("Sending the pokedex message...");
        let pokedex_msg = self
            .command
            .channel_id
            .send_message(&self.ctx, pokedex_msg_content)
            .await?;

        // Handle the interactions.
        self.handle_pokedex_interactions(pokedex_info, pokedex_msg)
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
    A method to send the empty pokedex message.
    */
    pub async fn send_empty_pokedex_msg(&self) -> HandlerResult<()> {
        info!("No pokemons found. Sending a message...");
        self.command
            .channel_id
            .send_message(&self.ctx, get_msg_pokedex_empty(&self.command.user.name))
            .await?;

        Ok(())
    }

    /**
    A method to handle the pokedex's interactions.

    ## Parameters:
    - `pokedex_info`: The pokedex information.
    - `pokedex_msg`: The pokedex message.
    */
    async fn handle_pokedex_interactions(
        &self,
        mut pokedex_info: PokedexInfo,
        pokedex_msg: Message,
    ) -> HandlerResult<()> {
        info!("Waiting for pokedex interactions...");

        let timeout = Duration::from_secs(EnvManager::get_var("BOT_POKEDEX_TIMEOUT_SECS"));
        let mut interactions = pokedex_msg
            .await_component_interaction(&self.ctx.shard)
            .timeout(timeout)
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
                "pokedex_previous" if pokedex_info.current_page > 1 => {
                    pokedex_info.current_page -= 1
                }
                "pokedex_next" if pokedex_info.current_page != pokedex_info.total_pages => {
                    pokedex_info.current_page += 1
                }
                _ => {
                    info!("Invalid interaction. Ignoring...");
                    debug!("Interaction: {}", interaction.data.custom_id.as_str());
                    continue;
                }
            }

            // Update the pokedex message.
            let embed = mount_pokedex_content_embed(&self.command.user, &mut pokedex_info).await?;
            let pokedex_msg_content = CreateInteractionResponseMessage::new()
                .add_embed(embed)
                .button(CreateButton::new("pokedex_previous").label("Previous"))
                .button(CreateButton::new("pokedex_next").label("Next"));

            // Send the pokedex message.
            interaction
                .create_response(
                    &self.ctx,
                    CreateInteractionResponse::UpdateMessage(pokedex_msg_content),
                )
                .await?;
        }

        Ok(())
    }
}

/**
A struct to store the pokedex information.
*/
struct PokedexInfo {
    pub total_pokes: u16,
    pub current_page: u16,
    pub total_pages: u16,
    pub pokes_per_page: u16,
    pub trainer_pokes: Vec<DBPoke>,
    pub trainer_species: Vec<DBPoke>,
    pub poke_cache: Vec<CachedPokemon>,
}

impl PokedexInfo {
    /**
    A method to create a new PokedexInfo.

    If the trainer has no pokemons, an Err will be returned.

    ## Parameters:
    - `command`: The command interaction.
    */
    async fn new(command: CommandInteraction) -> HandlerResult<PokedexInfo> {
        // Get the trainer and its pokemons.
        let trainer = {
            let user_id = command.user.id.to_string();
            let guild_id = command.guild_id.ok_or("Guild ID not found.")?;
            DBTrainer::find_by_discord_id(&user_id, &guild_id.to_string()).await?
        };
        let (trainer_pokes, trainer_species) = get_trainer_owned_pokes(trainer).await?;

        // Define the other pokedex information.
        let total_pokes = POKEFINDER_SERVICE.clone().get_poke_count().await?;
        let pokes_per_page = EnvManager::get_var("BOT_POKEDEX_POKES_PER_PAGE");
        let total_pages = trainer_species.len() as u16 / pokes_per_page + 1;
        let poke_cache = Vec::new();

        Ok(PokedexInfo {
            total_pokes,
            current_page: 1_u16,
            total_pages,
            pokes_per_page,
            trainer_pokes,
            trainer_species,
            poke_cache,
        })
    }
}
