// Libs
use super::{error::HandlerError, poke_spawn::PokeSpawnHandler};
use serenity::all::{Context, Message};
use std::sync::Arc;
use tracing::Instrument;

// Functions
/**
A method to spawn a new random pokémon.

## Parameters:
- `ctx`: A context of the current bot.
- `msg`: A message that triggered the command.
*/
pub async fn spawn_random_pokemon(
    ctx: Arc<Context>,
    msg: Arc<Message>,
) -> Result<(), HandlerError> {
    // Generate the Send the message to the channel.
    let gen_poke = PokeSpawnHandler::new(ctx.clone(), msg.channel_id);
    let (_cached_poke, poke_msg) = gen_poke.gen_and_send_poke().in_current_span().await?;

    // Start the capture of the pokemon.
    gen_poke.start_capture(poke_msg).in_current_span().await?;

    Ok(())
}
