use tracing::info;

use crate::{
    errors::DatabaseResult,
    models::{DBPoke, DBTrainer},
    services::DATABASE_SERVICE,
};

// Functions
/**
A method to find a list of all pokémons owned by a trainer.

## Parameters:
- `trainer`: The trainer to find the pokémons for.

## Returns:
- A tuple containing the list of all pokémons owned by the trainer and the list of all pokémon species owned by the trainer.
Both lists are sorted by the pokémon ID.
*/
pub async fn get_trainer_owned_pokes(
    trainer: DBTrainer,
) -> DatabaseResult<(Vec<DBPoke>, Vec<DBPoke>)> {
    // Define the query to fetch all pokemons owned by the trainer.
    info!("Finding all pokemons owned by the {}...", trainer.id);
    let db_svc = DATABASE_SERVICE.clone();
    let query = format!("SELECT * FROM pokemon WHERE trainer_id = '{}'", trainer.id);

    // Fetch the pokemons and sort them by their ID.
    let (trainer_pokes, trainer_species) = {
        let mut pokes: Vec<DBPoke> = db_svc.run_query(&query).await?;
        pokes.sort_by(|a, b| a.poke_id.cmp(&b.poke_id));
        let mut trainer_species = pokes.clone();
        trainer_species.dedup_by(|a, b| a.poke_id == b.poke_id);

        (pokes, trainer_species)
    };

    // Log the number of pokemons found and set the owned pokemons.
    info!("{} Pokemons found successfully.", trainer_pokes.len());
    Ok((trainer_pokes, trainer_species))
}
