// Libs
use super::error::HandlerError;
use crate::{
    messages::{
        get_msg_wild_pokemon_appeared, get_msg_wild_pokemon_caught, get_msg_wild_pokemon_fled,
    },
    serializations::cache::CachedPokemon,
    services::pokeapi::POKEAPI_SERVICE,
};
use once_cell::sync::Lazy;
use rand::Rng;
use serenity::{
    all::{parse_emoji, ChannelId, Context, User},
    model::channel::Message,
};
use std::sync::Arc;
use tracing::{info, Instrument};

// Data
const BOT_POKEBALL_EMOJI_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("BOT_POKEBALL_EMOJI_ID")
        .unwrap()
        .parse()
        .unwrap()
});
const BOT_POKE_SPAWN_RATE: Lazy<u16> = Lazy::new(|| {
    std::env::var("BOT_POKE_SPAWN_RATE")
        .unwrap()
        .parse::<u16>()
        .unwrap()
});
const BOT_POKE_FLEE_TIME_SECS: Lazy<u16> = Lazy::new(|| {
    std::env::var("BOT_POKE_FLEE_TIME_SECS")
        .unwrap()
        .parse::<u16>()
        .unwrap()
});

// Gen Poke Handler
pub struct PokeSpawnHandler {
    ctx: Arc<Context>,
    channel_id: ChannelId,
}

impl PokeSpawnHandler {
    /**
    A method to create a new PokeSpawnHandler.
    */
    pub fn new(ctx: Arc<Context>, channel_id: ChannelId) -> Self {
        PokeSpawnHandler { ctx, channel_id }
    }

    /**
    A method to handle the generation of a pokemon.
    */
    pub async fn handle(&self) -> Result<(), HandlerError> {
        // Try to generate a pokemon. If the chance was not met, return.
        if !will_poke_spawn() {
            return Ok(());
        }

        // Generate the Send the message to the channel.
        let (poke, poke_msg) = self.gen_and_send_poke().in_current_span().await?;

        // Start the capture of the pokemon.
        let new_trainer = self.start_capture(poke_msg).in_current_span().await?;
        if new_trainer.is_none() {
            info!("The pokemon has fled!");
            let message = get_msg_wild_pokemon_fled(&poke);
            self.channel_id
                .send_message(&self.ctx.http, message)
                .await?;
            return Ok(());
        }

        // Save the pokemon to the trainer's pokedex.
        let message = get_msg_wild_pokemon_caught(&poke, new_trainer.unwrap());
        self.channel_id
            .send_message(&self.ctx.http, message)
            .await?;
        info!("TODO");

        Ok(())
    }

    /**
    A method to try to generate a new pokemon.

    ## Returns:
    - A message with the pokemon.
    */
    pub async fn gen_and_send_poke(&self) -> Result<(CachedPokemon, Message), HandlerError> {
        // Generate a pokemon and send the message to the channel.
        let poke_svc = POKEAPI_SERVICE.clone();
        let poke_amount = poke_svc.get_pokemons_count().in_current_span().await?;
        let poke_id = rand::thread_rng().gen_range(1..=poke_amount).to_string();
        let poke = poke_svc.find_poke(&poke_id).in_current_span().await?;

        let message = get_msg_wild_pokemon_appeared(&poke);
        let poke_message = self
            .channel_id
            .send_message(&self.ctx.http, message)
            .in_current_span()
            .await?;

        // React to the message with the pokeball.
        let emoji = parse_emoji(&BOT_POKEBALL_EMOJI_ID.to_string()).unwrap();
        poke_message.react(&self.ctx.http, emoji).await?;

        info!("A pokemon has spawned!");
        Ok((poke, poke_message))
    }

    /**
    A method to start the capture of a pokemon.
    */
    pub async fn start_capture(&self, poke_msg: Message) -> Result<Option<User>, HandlerError> {
        // Start a instant capture of the pokemon.
        let start_instant = std::time::Instant::now();

        // Check each second if the pokemon has fled.
        while start_instant.elapsed().as_secs() < *BOT_POKE_FLEE_TIME_SECS as u64 {
            // Check the reactions of the message.
            info!("Checking the reactions of the message...");
            let emoji = parse_emoji(&BOT_POKEBALL_EMOJI_ID.to_string()).unwrap();
            let users_that_reacted = &poke_msg
                .reaction_users(&self.ctx.http, emoji, Some(2), None)
                .await?;

            // If someone reacted to the message, return.
            if users_that_reacted.len() > 1 {
                info!("The pokemon has been captured!");
                return Ok(Some(
                    users_that_reacted[users_that_reacted.len() - 2].clone(),
                ));
            }

            // Wait for a second.
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Ok(None)
    }
}

// Functions
/**
A method to to check the chance of a pokemon to be spawned.
*/
fn will_poke_spawn() -> bool {
    info!("Checking the chance of a pokemon to be spawned...");
    let chance = rand::thread_rng().gen_range(0..*BOT_POKE_SPAWN_RATE);
    match chance {
        0 => {
            info!("A pokemon will be spawned!");
            true
        }
        _ => {
            info!("A pokemon will not be spawned.");
            false
        }
    }
}
