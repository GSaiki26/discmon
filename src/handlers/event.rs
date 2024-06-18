// Libs
use async_trait::async_trait;
use serenity::all::{
    Command, CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, Message,
};
use std::sync::Arc;
use tracing::{error, info, Instrument};

use crate::handlers::pokedex::PokedexHandler;

use super::poke_spawn::PokeSpawnHandler;

#[cfg(feature = "dev_commands")]
use super::dev_commands::DevCommandsHandler;

// Event Handler
pub struct EventHandler;

#[async_trait]
impl serenity::all::EventHandler for EventHandler {
    /**
    A method to handle the ready event.

    ## Parameters:
    - `ctx`: A `Context` type. The context of the event.
    - `ready`: A `Ready` type. The ready event.
    */
    async fn ready(&self, ctx: Context, _ready: serenity::model::gateway::Ready) {
        // Create the interactive commands.
        let pokedex_command = CreateCommand::new("pokedex")
            .description("A command to show your current pokedex")
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "only_shinies",
                "If you want to see only the shinies",
            ));
        if let Err(e) = Command::create_global_command(&ctx.http, pokedex_command).await {
            error!("Error creating the pokedex command: {:?}", e);
        }
    }

    /**
    A method to handle the interactions.

    ## Parameters:
    - `ctx`: A `Context` type. The context of the event.
    - `interaction`: A `Interaction` type. The interaction that triggered the event.
    */
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let ctx = Arc::new(ctx);

        // Check if the interaction is a command.
        let command = match interaction {
            Interaction::Command(command) if command.guild_id.is_some() => command,
            _ => return,
        };

        let span = tracing::info_span!(
            "message_handler",
            author = command.user.id.to_string(),
            guild = command.guild_id.as_ref().unwrap().to_string(),
        );
        let _guard = span.enter();

        info!("A user started a new command.");
        if command.data.name.as_str() == "pokedex" {
            let pokedex_handler = PokedexHandler::new(ctx, command);
            if let Err(e) = pokedex_handler.handle().in_current_span().await {
                error!("Error handling pokedex command: {:?}", e);
            }
        };
    }

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
                let dev_handler =
                    DevCommandsHandler::new(ctx, msg.channel_id, msg.guild_id.unwrap());
                if let Err(e) = dev_handler.handle(msg).in_current_span().await {
                    error!("Error handling dev command: {:?}", e);
                }
                return;
            }
        }

        // Pass the handle responsibility to the PokeSpawnHandler.
        let pokespawn_handler =
            PokeSpawnHandler::new(ctx.clone(), msg.channel_id, msg.guild_id.unwrap());
        if let Err(e) = pokespawn_handler.handle().await {
            error!("Error handling message: {:?}", e);
        }
    }
}
