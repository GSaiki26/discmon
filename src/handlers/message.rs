// Libs
use async_trait::async_trait;
use serenity::{
    model::channel::Message,
    prelude::{Context, EventHandler},
};
use std::sync::Arc;
use tracing::error;

use super::poke_spawn::PokeSpawnHandler;

#[cfg(feature = "dev_commands")]
use super::dev_commands::DevCommandsHandler;

// Message Handler
pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    /**
    A method to handle messages. Here the bot will generate a chance for spawning a pokemon.

    ## Parameters:
    - `ctx`: A `Context` type. The context of the event.
    - `msg`: A `Message` type. The message that triggered the event.
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

        // Check if the message is a command. It'll only be available if the feature is enabled.
        #[cfg(feature = "dev_commands")]
        {
            if msg.content.starts_with("!dev") {
                let dev_handler = DevCommandsHandler::new(ctx, msg.channel_id);
                if let Err(e) = dev_handler.handle(msg).await {
                    error!("Error handling dev command: {:?}", e);
                }
                return;
            }
        }

        // Pass the handle responsibility to the PokeSpawnHandler.
        let pokespawn_handler = PokeSpawnHandler::build(ctx.clone(), msg.channel_id).build();
        if let Err(e) = pokespawn_handler.handle().await {
            error!("Error handling message: {:?}", e);
        }
    }
}
