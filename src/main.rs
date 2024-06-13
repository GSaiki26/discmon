// Libs
// use services::pokeapi::{get_poke, get_poke_evolution_chain, get_poke_species};
// use tracing::info;
mod serializations;
mod services;

// Functions
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
}

// info!("Getting poke...");
// let poke_1 = get_poke("1").await.unwrap();

// info!("Getting poke species...");
// let poke_1_species = get_poke_species(&poke_1.species.name).await.unwrap();

// info!("Getting poke evolution chain...");
// // Reverse the chain url and get the second element.
// let chain: Vec<&str> = poke_1_species.evolution_chain.url.split("/").collect();
// let chain: u16 = chain[chain.len() - 2].parse().unwrap();
// let poke_1_evolution_chain = get_poke_evolution_chain(&chain).await.unwrap();

// info!("{:?}", poke_1);
// info!("{:?}", poke_1_species);
// info!("{:?}", poke_1_evolution_chain);
