// Libs
use rand::Rng;
use serenity::{
    all::{parse_emoji, ChannelId, Context, EmojiIdentifier, Mentionable, User},
    model::channel::Message,
};
use std::{env::var, sync::Arc};
use tracing::{info, Instrument};

use crate::services::POKEAPI_SERVICE;
use crate::{errors::HandlerError, serializations::handlers::SpawnedPokemon};
use crate::{
    messages::{
        get_msg_wild_pokemon_appeared, get_msg_wild_pokemon_caught, get_msg_wild_pokemon_fled,
    },
    serializations::cache::CachedPokemon,
};

// Gen Poke Handler
pub struct PokeSpawnHandler {
    ctx: Arc<Context>,
    channel_id: ChannelId,

    bot_pokeball_emoji: EmojiIdentifier,
    bot_poke_spawn_rate: u64,
    bot_poke_flee_time_secs: u64,
    bot_poke_shiny_rate: u64,
}

impl PokeSpawnHandler {
    /**
    A method to build a new PokeSpawnHandler.
    */
    pub fn build(ctx: Arc<Context>, channel_id: ChannelId) -> PokeSpawnHandlerBuilder {
        PokeSpawnHandlerBuilder::new(ctx, channel_id)
    }

    /**
    A method to handle the generation of a pokemon. If the chance was not met, return.

    Its possible to pass a custom chance to the method, being 1 to X, where X is the chance of the pokemon to be spawned.
    */
    pub async fn handle(&self) -> Result<(), HandlerError> {
        // Check the chance of the pokemon to be spawned.
        info!(
            "[PokeSpawnHandler] Checking the chance 1 to {} of a pokemon to be spawned...",
            self.bot_poke_spawn_rate
        );
        let chance = rand::thread_rng().gen_range(0..self.bot_poke_spawn_rate);
        if chance != 0 {
            info!("A pokemon will not be spawned.");
            return Ok(());
        }

        // Spawn a new pokemon and send it to the channel.
        info!("[PokeSpawnHandler] The chance was met! Spawning a new pokemon...");
        let spawned_poke = self.gen_poke().in_current_span().await?;
        let cached_poke = POKEAPI_SERVICE
            .clone()
            .find_poke(&spawned_poke.id.to_string())
            .in_current_span()
            .await?;
        let poke_msg = self
            .send_poke_message(&spawned_poke, &cached_poke)
            .in_current_span()
            .await?;

        // Start the capture of the pokemon.
        let new_trainer = self.start_capture(poke_msg).in_current_span().await?;
        if new_trainer.is_none() {
            info!("[PokeSpawnHandler] The pokemon has fled!");
            let message = get_msg_wild_pokemon_fled(spawned_poke.is_shiny, &cached_poke.name);
            self.channel_id
                .send_message(&self.ctx.http, message)
                .await?;
            return Ok(());
        }
        let new_trainer = new_trainer.unwrap();

        // Save the pokemon to the trainer's pokedex.
        let message = get_msg_wild_pokemon_caught(
            spawned_poke.is_shiny,
            &cached_poke.name,
            new_trainer.mention(),
        );
        self.channel_id
            .send_message(&self.ctx.http, message)
            .await?;
        info!("TODO");

        Ok(())
    }

    /**
    A method to generate a pokemon.

    ## Returns:
    - A `SpawnedPokemon` type. The generated pokemon.
    */
    pub async fn gen_poke(&self) -> Result<SpawnedPokemon, HandlerError> {
        info!("[PokeSpawnHandler] Generating a new pokemon...");
        let poke_svc = POKEAPI_SERVICE.clone();
        let poke_amount = poke_svc.get_pokemons_count().in_current_span().await?;
        let poke_id = rand::thread_rng().gen_range(1..=poke_amount);
        let is_shiny = rand::thread_rng().gen_range(0..self.bot_poke_shiny_rate) == 0;

        info!(
            "[PokeSpawnHandler] The pokemon #{} has been generated!",
            poke_id
        );
        Ok(SpawnedPokemon::new(poke_id, is_shiny))
    }

    /**
    A method to send the pokemon to the channel.

    ## Parameters:
    - `poke`: The pokemon to be sent.
    */
    pub async fn send_poke_message(
        &self,
        spawned_poke: &SpawnedPokemon,
        cached_poke: &CachedPokemon,
    ) -> Result<Message, HandlerError> {
        info!("[PokeSpawnHandler] Sending the pokemon to the channel...");
        let message = get_msg_wild_pokemon_appeared(spawned_poke.is_shiny, cached_poke);
        let poke_message = self
            .channel_id
            .send_message(&self.ctx.http, message)
            .in_current_span()
            .await?;

        info!("[PokeSpawnHandler] Reacting to the message with the pokeball emoji...");
        poke_message
            .react(&self.ctx.http, self.bot_pokeball_emoji.clone())
            .await?;

        info!("[PokeSpawnHandler] The pokemon has been sent to the channel!");
        Ok(poke_message)
    }

    /**
    A method to start the capture of a pokemon.
    */
    pub async fn start_capture(&self, poke_msg: Message) -> Result<Option<User>, HandlerError> {
        info!("[PokeSpawnHandler] Starting the capture event of the pokemon...");
        let start_instant = std::time::Instant::now();

        while start_instant.elapsed().as_secs() < self.bot_poke_flee_time_secs as u64 {
            // Check the reactions of the message.
            info!("[PokeSpawnHandler] Checking the reactions of the message...");
            let users_that_reacted = &poke_msg
                .reaction_users(
                    &self.ctx.http,
                    self.bot_pokeball_emoji.clone(),
                    Some(2),
                    None,
                )
                .await?;

            // If someone reacted to the message, return.
            if users_that_reacted.len() > 1 {
                info!("[PokeSpawnHandler] The pokemon has been captured!");
                return Ok(Some(
                    users_that_reacted[users_that_reacted.len() - 2].clone(),
                ));
            }

            // Wait for a second.
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        info!("[PokeSpawnHandler] The pokemon has fled!");
        Ok(None)
    }
}

// Builder
#[derive(Debug)]
pub struct PokeSpawnHandlerBuilder {
    ctx: Arc<Context>,
    channel_id: ChannelId,

    bot_pokeball_emoji: Option<EmojiIdentifier>,
    bot_poke_spawn_rate: Option<u64>,
    bot_poke_flee_time_secs: Option<u64>,
    bot_poke_shiny_rate: Option<u64>,
}

impl PokeSpawnHandlerBuilder {
    /**
    A method to create a new PokeSpawnHandlerBuilder.
    */
    pub fn new(ctx: Arc<Context>, channel_id: ChannelId) -> Self {
        Self {
            ctx,
            channel_id,
            bot_pokeball_emoji: None,
            bot_poke_spawn_rate: None,
            bot_poke_flee_time_secs: None,
            bot_poke_shiny_rate: None,
        }
    }

    /**
    A method to set the bot pokeball emoji id.

    ## Parameters:
    - `bot_pokeball_emoji_id`: A `String` type. The bot pokeball emoji id.
    */
    pub fn with_pokeball_emoji_id(mut self, bot_pokeball_emoji_id: String) -> Self {
        self.bot_pokeball_emoji = parse_emoji(bot_pokeball_emoji_id);
        self
    }

    /**
    A method to set the bot poke spawn rate.

    ## Parameters:
    - `bot_poke_spawn_rate`: A `u64` type. The bot poke spawn rate.
    */
    pub fn with_poke_spawn_rate(mut self, bot_poke_spawn_rate: u64) -> Self {
        self.bot_poke_spawn_rate = Some(bot_poke_spawn_rate);
        self
    }

    /**
    A method to set the bot poke flee time seconds.

    ## Parameters:
    - `bot_poke_flee_time_secs`: A `u64` type. The bot poke flee time seconds.
    */

    pub fn with_poke_flee_time_secs(mut self, bot_poke_flee_time_secs: u64) -> Self {
        self.bot_poke_flee_time_secs = Some(bot_poke_flee_time_secs);
        self
    }

    /**
    A method to set the bot poke shiny rate.

    ## Parameters:
    - `bot_poke_shiny_rate`: A `u64` type. The bot poke shiny rate.
    */
    pub fn with_poke_shiny_rate(mut self, bot_poke_shiny_rate: u64) -> Self {
        self.bot_poke_shiny_rate = Some(bot_poke_shiny_rate);
        self
    }

    /**
    A method to build a new PokeSpawnHandler.
    */
    pub fn build(self) -> PokeSpawnHandler {
        PokeSpawnHandler {
            ctx: self.ctx,
            channel_id: self.channel_id,
            bot_pokeball_emoji: self
                .bot_pokeball_emoji
                .unwrap_or_else(|| parse_emoji(var("BOT_POKEBALL_EMOJI_ID").unwrap()).unwrap()),
            bot_poke_spawn_rate: self.bot_poke_spawn_rate.unwrap_or_else(|| {
                var("BOT_POKEBALL_EMOJI_ID")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            }),
            bot_poke_flee_time_secs: self.bot_poke_flee_time_secs.unwrap_or_else(|| {
                var("BOT_POKEBALL_EMOJI_ID")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            }),
            bot_poke_shiny_rate: self.bot_poke_shiny_rate.unwrap_or_else(|| {
                var("BOT_POKEBALL_EMOJI_ID")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            }),
        }
    }
}
