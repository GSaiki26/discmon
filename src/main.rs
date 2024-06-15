// Libs
use serenity::all::GatewayIntents;
// use services::{DATABASE_SERVICE, POKEAPI_SERVICE};
use services::POKEAPI_SERVICE;
use std::process::exit;
use tracing::error;

use crate::handlers::message::MessageHandler;

mod di;
mod errors;
mod handlers;
mod messages;
mod serializations;
mod services;

// Functions
/**
A method to initialize the services.

It'll initialize the services and try to connect to its dependencies.
*/
async fn init_services() {
    // Get the PokeAPI service. I'll gracefully exit if it fails.
    let pokeapi_svc = POKEAPI_SERVICE.clone();
    if let Err(e) = pokeapi_svc.connect_to_cache().await {
        error!("Error connecting to the cache. {}", e);
        exit(1);
    }

    // // Get the Database service. I'll gracefully exit if it fails.
    // let db_svc = DATABASE_SERVICE.clone();
    // if let Err(e) = db_svc.connect().await {
    //     error!("Error connecting to the database. {}", e);
    //     exit(1);
    // };
}

/**
A method to get the discord's bot client.
*/
async fn get_client() -> Result<serenity::Client, serenity::Error> {
    // Define the permissions for the server.
    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS;

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    serenity::Client::builder(token, intents)
        .event_handler(MessageHandler)
        .await
}

#[tokio::main]
async fn main() {
    // Initialize the services.
    tracing_subscriber::fmt::init();

    // Initialize the services.
    init_services().await;

    // Get the discord's bot client.
    let mut client = match get_client().await {
        Ok(client) => client,
        Err(e) => {
            error!("Error creating client: {:?}", e);
            exit(1);
        }
    };

    // Start the client.
    if let Err(e) = client.start().await {
        error!("Error starting client: {:?}", e);
    }
}
