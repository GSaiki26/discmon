// Libs
use crate::handlers::message::MessageHandler;
use di::{
    cache::{CacheError, RedisCache},
    http_client::ReqwestHTTPClient,
};
use serenity::all::GatewayIntents;
use services::pokeapi::{PokeAPI, POKEAPI_SERVICE};
use std::{process::exit, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tracing::error;
mod di;
mod handlers;
mod messages;
mod serializations;
mod services;

// Functions
/**
A method to a new instance of the PokeAPI service.
*/
pub fn get_pokeapi_svc() -> Result<PokeAPI<RedisCache, ReqwestHTTPClient>, CacheError> {
    let redis_instance = RedisCache::new()?;
    let reqwest_instance = ReqwestHTTPClient::new();
    Ok(PokeAPI::new(
        Arc::new(Mutex::new(redis_instance)),
        Arc::new(RwLock::new(reqwest_instance)),
    ))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Get the PokeAPI service. I'll gracefully exit if it fails.
    let pokeapi_svc = POKEAPI_SERVICE.clone();
    if let Err(e) = pokeapi_svc.connect_to_cache().await {
        error!("Error connecting to the cache: {:?}", e);
        exit(1);
    }

    // Define the permissions for the server.
    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS;

    let token = match std::env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("No token found in environment variables.");
            exit(1);
        }
    };
    let mut client = serenity::Client::builder(token, intents)
        .event_handler(MessageHandler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        error!("Error starting client: {:?}", e);
    }
}
