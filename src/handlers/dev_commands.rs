// Libs
use serenity::all::{ChannelId, Context, GuildId, Message};
use std::sync::Arc;
use tracing::info;

use super::poke_spawn::PokeSpawnHandler;
use crate::{
    errors::{HandlerError, HandlerResult},
    messages::get_dev_error_msg_poke_spawn,
};

// DevCommandsHandler
#[cfg(feature = "dev_commands")]
pub struct DevCommandsHandler {
    ctx: Arc<Context>,
    channel_id: ChannelId,
    guild_id: GuildId,
}

#[cfg(feature = "dev_commands")]
impl DevCommandsHandler {
    /**
    A method to create a new DevCommandsHandler.

    ## Parameters:
    - `ctx`: An `Arc<Context>` type. The context of the event.
    - `channel_id`: A `ChannelId` type. The channel id of the event.
    */
    pub fn new(ctx: Arc<Context>, channel_id: ChannelId, guild_id: GuildId) -> Self {
        DevCommandsHandler {
            ctx,
            channel_id,
            guild_id,
        }
    }

    /**
    A method to handle the dev commands.

    To this Handler be called, it must unsure that the prefix is `!dev`.

    ## Parameters:
    - `msg`: A `Message` type. The message that triggered the event.
    */
    pub async fn handle(&self, msg: Arc<Message>) -> HandlerResult<()> {
        info!("Handling dev command...");
        if msg.content.starts_with("!dev spawn_random") {
            let args = msg.content.split_whitespace().collect::<Vec<&str>>();
            if let Err(e) = self.handle_spawn_random_command(args).await {
                info!("Error handling the command. {}", e);
                let error_message = get_dev_error_msg_poke_spawn();
                self.channel_id
                    .send_message(&self.ctx.http, error_message)
                    .await?;
            }
        }

        info!("Dev command handled.");
        Ok(())
    }

    /**
    A method to handle the spawn_random command.

    ## Parameters:
    - `message`: A `Message` type. The message that triggered the event.
    */
    async fn handle_spawn_random_command(&self, args: Vec<&str>) -> HandlerResult<()> {
        info!("Checking arguments...");
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

        info!("Spawning a random pokemon...");
        self.spawn_random_pokemon(poke_flee_time_secs, poke_shiny_rate)
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
    ) -> HandlerResult<()> {
        // Create the PokeSpawnHandler with custom values.
        let shiny_rate = if shiny_rate { 1 } else { 10000 };
        let mut poke_spawn_handler =
            PokeSpawnHandler::new(self.ctx.clone(), self.channel_id, self.guild_id);
        poke_spawn_handler.bot_poke_spawn_rate = 1;
        poke_spawn_handler.bot_poke_flee_time_secs = flee_time_secs;
        poke_spawn_handler.bot_poke_shiny_rate = shiny_rate;

        // Generate the Send the message to the channel.
        poke_spawn_handler.handle().await?;

        Ok(())
    }
}
