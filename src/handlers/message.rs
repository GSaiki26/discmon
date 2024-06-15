// Libs
use super::poke_spawn::PokeSpawnHandler;
use async_trait::async_trait;
use serenity::{
    model::channel::Message,
    prelude::{Context, EventHandler},
};
use std::sync::Arc;
use tracing::error;

#[cfg(feature = "dev_commands")]
use super::dev_commands::spawn_random_pokemon;

// Message Handler
pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    /**
    A method to handle messages. Here the bot will generate a chance for spawning a pokemon.
    */
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots
        let ctx = Arc::new(ctx);
        let msg = Arc::new(msg);
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }

        let span = tracing::info_span!(
            "message_handler",
            author = msg.author.id.to_string(),
            guild = msg.guild_id.unwrap().to_string(),
        );
        let _guard = span.enter();

        #[cfg(feature = "dev_commands")]
        {
            // Check if the message is a command.
            if msg.content.starts_with("!dev") {
                match msg.content.as_str() {
                    "!dev spawn_random" => {
                        if let Err(e) = spawn_random_pokemon(ctx.clone(), msg.clone()).await {
                            error!("Error spawning random pokemon: {:?}", e);
                        }
                    }
                    _ => return,
                }
            }
        }

        if let Err(e) = PokeSpawnHandler::new(ctx.clone(), msg.channel_id)
            .handle()
            .await
        {
            error!("Error handling message: {:?}", e);
        }
    }
}
