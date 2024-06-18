// Libs
use rand::Rng;
use serenity::{
    all::{parse_emoji, ChannelId, Context, EmojiIdentifier, GuildId, Mention, Mentionable, User},
    model::channel::Message,
};
use std::sync::Arc;
use tracing::info;

use crate::{
    errors::HandlerResult,
    models::{DBModel, DBPoke, DBTrainer},
};
use crate::{
    messages::{
        get_msg_wild_pokemon_appeared, get_msg_wild_pokemon_caught, get_msg_wild_pokemon_fled,
    },
    serializations::cache::CachedPokemon,
};
use crate::{services::POKEFINDER_SERVICE, utils::EnvManager};

// Gen Poke Handler
/**
A struct to handle the generation of a pokemon.
*/
pub struct PokeSpawnHandler {
    ctx: Arc<Context>,
    channel_id: ChannelId,
    guild_id: GuildId,

    pub bot_pokeball_emoji: EmojiIdentifier,
    pub bot_poke_spawn_rate: u64,
    pub bot_poke_flee_time_secs: u64,
    pub bot_poke_shiny_rate: u64,
}

impl PokeSpawnHandler {
    /**
    A method to create a new PokeSpawnHandler.
    */
    pub fn new(ctx: Arc<Context>, channel_id: ChannelId, guild_id: GuildId) -> Self {
        Self {
            ctx,
            channel_id,
            guild_id,
            bot_pokeball_emoji: parse_emoji(EnvManager::get_var::<String>("BOT_POKEBALL_EMOJI_ID"))
                .unwrap(),
            bot_poke_spawn_rate: EnvManager::get_var("BOT_POKE_SPAWN_RATE"),
            bot_poke_flee_time_secs: EnvManager::get_var("BOT_POKE_FLEE_TIME_SECS"),
            bot_poke_shiny_rate: EnvManager::get_var("BOT_POKE_SHINY_RATE"),
        }
    }

    /**
    A method to handle the generation of a pokemon. If the chance was not met, return.
    */
    pub async fn handle(&self) -> HandlerResult<()> {
        // Check the chance of the pokemon to be spawned.
        info!(
            "Checking the chance 1 to {} of a pokemon to be spawned...",
            self.bot_poke_spawn_rate
        );
        if !self.can_spawn() {
            info!("The chance was not met, a pokemon will not be spawned.");
            return Ok(());
        }

        // Spawn a new pokemon and send it to the channel.
        info!("The chance was met! Spawning a new pokemon...");
        let spawned_poke_info = self.generate_new_poke().await?;
        let cached_poke = POKEFINDER_SERVICE
            .clone()
            .find_poke(&spawned_poke_info.1.to_string())
            .await?;

        // // Check the spawned pokemon's emoji.
        // self.create_poke_emoji(&cached_poke).await?;

        // Send the pokemon to the channel.
        info!("Sending the pokemon to the channel...");
        let poke_msg = self
            .create_poke_msg(spawned_poke_info.0, &cached_poke)
            .await?;

        // Start the capture event of the pokemon.
        info!("Starting the capture event of the pokemon...");
        let user_who_captured = self.start_capture(&poke_msg).await?;
        self.send_final_capture_msg(
            spawned_poke_info.0,
            &cached_poke.name,
            user_who_captured.as_ref().map(|user| user.mention()),
        )
        .await?;
        if user_who_captured.is_none() {
            return Ok(());
        }

        // Save the pokemon to the trainer's pokedex.
        let trainer_id = user_who_captured.unwrap().id.to_string();
        let trainer =
            DBTrainer::find_by_discord_id(&trainer_id, &self.guild_id.to_string()).await?;
        let mut poke = DBPoke::new(&trainer.id, &spawned_poke_info.1, spawned_poke_info.0);
        poke.create().await?;

        Ok(())
    }

    /**
    A method to check a pokemon can be spawned.
    It uses the bot_poke_spawn_rate to check if a pokemon can be spawned.
    */
    pub fn can_spawn(&self) -> bool {
        let chance = rand::thread_rng().gen_range(0..self.bot_poke_spawn_rate);
        chance == 0
    }

    /**
    A method to generate a new pokemon to be spawned.

    ## Returns:
    - A (is_shiny, poke_id) tuple.
    */
    pub async fn generate_new_poke(&self) -> HandlerResult<(bool, u16)> {
        let poke_svc = POKEFINDER_SERVICE.clone();
        let poke_amount = poke_svc.get_poke_count().await?;
        let poke_id = rand::thread_rng().gen_range(1..=poke_amount);
        let is_shiny = rand::thread_rng().gen_range(0..self.bot_poke_shiny_rate) == 0;
        Ok((is_shiny, poke_id))
    }

    // /**
    // A method to create the emoji of a pokemon.
    // */
    // pub async fn create_poke_emoji(&self, cached_poke: &CachedPokemon) -> HandlerResult<()> {
    //     // Check if the emoji already exists.
    //     info!("Checking if the pokemon emoji already exists...");
    //     for emoji in self.guild_id.emojis(&self.ctx.http).await? {
    //         if emoji.name == format!("pokemon_{}", cached_poke.name)
    //             || emoji.name == format!("pokemon_{}_shiny", cached_poke.name)
    //         {
    //             return Ok(());
    //         }
    //     }

    //     // Create the pokemon emoji.
    //     info!("Creating the pokemon emoji...");
    //     let emoji_name = format!("pokemon_{}", cached_poke.name);
    //     let shiny_emoji_name = format!("pokemon_{}_shiny", cached_poke.name);
    //     let image =
    //         CreateAttachment::url(&self.ctx.http, &cached_poke.sprites.front_default).await?;
    //     let shiny_image =
    //         CreateAttachment::url(&self.ctx.http, &cached_poke.sprites.front_shiny).await?;

    //     self.guild_id
    //         .create_emoji(&self.ctx.http, &emoji_name, &image.to_base64())
    //         .await?;
    //     self.guild_id
    //         .create_emoji(&self.ctx.http, &shiny_emoji_name, &shiny_image.to_base64())
    //         .await?;

    //     Ok(())
    // }

    /**
    A method to create a message with the pokemon information.

    ## Parameters:
    - `is_shiny`: A `bool` type. If the pokemon is shiny.
    - `cached_poke`: A `CachedPokemon` type. The cached pokemon.

    ## Returns:
    - A `Message` type.
    */
    pub async fn create_poke_msg(
        &self,
        is_shiny: bool,
        cached_poke: &CachedPokemon,
    ) -> HandlerResult<Message> {
        let message = get_msg_wild_pokemon_appeared(is_shiny, cached_poke);
        let message = self
            .channel_id
            .send_message(&self.ctx.http, message)
            .await?;
        message
            .react(&self.ctx.http, self.bot_pokeball_emoji.clone())
            .await?;

        Ok(message)
    }

    /**
    A method to start the capture of a pokemon.

    ## Parameters:
    - `poke_msg`: A `Message` type. The message of the pokemon.

    ## Returns:
    - A `Option<User>` type. The user who captured the pokemon.
    */
    pub async fn start_capture(&self, poke_msg: &Message) -> HandlerResult<Option<User>> {
        let start_instant = std::time::Instant::now();

        while start_instant.elapsed().as_secs() < self.bot_poke_flee_time_secs {
            // Check the reactions of the message.
            info!("Checking the reactions of the message...");
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
                info!("The pokemon has been captured!");
                return Ok(Some(
                    users_that_reacted[users_that_reacted.len() - 2].clone(),
                ));
            }

            // Wait for a second.
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        info!("The pokemon has fled!");
        Ok(None)
    }

    /**
    A method to send the final message of a pokemon capture.

    ## Parameters:
    - `is_shiny`: A `bool` type. If the pokemon is shiny.
    - `poke_name`: A `&str` type. The name of the pokemon.
    - `user_mention`: A `Option<Mention>` type. The mention of the user who captured the pokemon.
    */
    pub async fn send_final_capture_msg(
        &self,
        is_shiny: bool,
        poke_name: &str,
        user_mention: Option<Mention>,
    ) -> HandlerResult<()> {
        match user_mention {
            Some(user_mention) => {
                let message = get_msg_wild_pokemon_caught(is_shiny, poke_name, user_mention);
                self.channel_id
                    .send_message(&self.ctx.http, message)
                    .await?;
            }
            None => {
                let message = get_msg_wild_pokemon_fled(is_shiny, poke_name);
                self.channel_id
                    .send_message(&self.ctx.http, message)
                    .await?;
            }
        }

        Ok(())
    }
}
