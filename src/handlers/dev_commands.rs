// Libs
use serenity::all::{ChannelId, Context, Message};
use std::sync::Arc;
use tracing::{info, Instrument};

use super::poke_spawn::PokeSpawnHandler;
use crate::{errors::HandlerError, messages::get_dev_error_msg_poke_spawn};

// DevCommandsHandler
#[cfg(feature = "dev_commands")]
pub struct DevCommandsHandler {
    ctx: Arc<Context>,
    channel_id: ChannelId,
}

#[cfg(feature = "dev_commands")]
impl DevCommandsHandler {
    /**
    A method to create a new DevCommandsHandler.

    ## Parameters:
    - `ctx`: An `Arc<Context>` type. The context of the event.
    - `channel_id`: A `ChannelId` type. The channel id of the event.
    */
    pub fn new(ctx: Arc<Context>, channel_id: ChannelId) -> Self {
        DevCommandsHandler { ctx, channel_id }
    }

    /**
    A method to handle the dev commands.

    To this Handler be called, it must unsure that the prefix is `!dev`.

    ## Parameters:
    - `msg`: A `Message` type. The message that triggered the event.
    */
    pub async fn handle(&self, msg: Arc<Message>) -> Result<(), HandlerError> {
        info!("[DEV HANDLER] Handling dev command...");
        if msg.content.starts_with("!dev spawn_random") {
            let args = msg.content.split_whitespace().collect::<Vec<&str>>();
            if let Err(e) = self.handle_spawn_random_command(args).await {
                info!("[DEV HANDLER] Error handling the command. {}", e);
                let error_message = get_dev_error_msg_poke_spawn();
                self.channel_id
                    .send_message(&self.ctx.http, error_message)
                    .await?;
            }
        }

        info!("[DEV HANDLER] Dev command handled.");
        Ok(())
    }

    /**
    A method to handle the spawn_random command.

    ## Parameters:
    - `message`: A `Message` type. The message that triggered the event.
    */
    async fn handle_spawn_random_command(&self, args: Vec<&str>) -> Result<(), HandlerError> {
        info!("[DEV HANDLER] Checking arguments...");
        if args.len() != 4 {
            return Err(HandlerError::Other("Invalid arguments.".to_string()));
        };

        let (poke_flee_time_secs, poke_shiny_rate) =
            match (args[2].parse::<u64>(), args[3].parse::<bool>()) {
                (Ok(poke_flee_time_secs), Ok(poke_shiny_rate)) => {
                    (poke_flee_time_secs, poke_shiny_rate)
                }
                _ => {
                    return Err(HandlerError::Other("Invalid arguments.".to_string()));
                }
            };

        info!("[DEV HANDLER] Spawning a random pokemon...");
        self.spawn_random_pokemon(poke_flee_time_secs, poke_shiny_rate)
            .in_current_span()
            .await?;

        Ok(())
    }

    /**
    A method to spawn a new random pokémon.
    */
    async fn spawn_random_pokemon(
        &self,
        flee_time_secs: u64,
        shiny_rate: bool,
    ) -> Result<(), HandlerError> {
        // Create the PokeSpawnHandler with custom values.
        let shiny_rate = if shiny_rate { 1 } else { 10000 };
        let poke_spawn_handler = PokeSpawnHandler::build(self.ctx.clone(), self.channel_id)
            .with_poke_spawn_rate(1)
            .with_poke_flee_time_secs(flee_time_secs)
            .with_poke_shiny_rate(shiny_rate)
            .build();

        // Generate the Send the message to the channel.
        poke_spawn_handler.handle().await?;

        Ok(())
    }
}
