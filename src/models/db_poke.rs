// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::info;

use super::DBModel;
use crate::{errors::DatabaseResult, services::DATABASE_SERVICE};

// DBPoke
/**
A struct to represent a Pokemon in the database.
*/
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBPoke {
    pub id: Thing,
    pub trainer_id: Thing,
    pub poke_id: u16,
    pub is_shiny: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl DBPoke {
    /**
    A method to create a new instance of DatabasePokemon.

    # Parameters:
    - `trainer_id`: The ID of the trainer who owns the pokemon.
    - `poke_id`: The ID of the pokemon.
    - `is_shiny`: A boolean to ensure if the pokemon is shiny or not.
    */
    pub fn new(trainer_id: &Thing, poke_id: &u16, is_shiny: bool) -> Self {
        Self {
            id: Thing {
                tb: String::from("pokemon"),
                id: Id::ulid(),
            },
            trainer_id: trainer_id.clone(),
            is_shiny,
            poke_id: *poke_id,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }

    /**
    A method to find a list of all pokémons owned by a trainer.

    ## Parameters:
    - `trainer_id`: The ID of the trainer.
    */
    pub async fn find_by_trainer_id(trainer_id: &Thing) -> DatabaseResult<Vec<Self>> {
        info!("Finding all pokemons owned by the {}...", trainer_id);
        let db_svc = DATABASE_SERVICE.clone();
        let query = format!("SELECT * FROM pokemon WHERE trainer_id = '{}'", trainer_id);
        let pokes = db_svc.run_query(&query).await?;

        info!("{} Pokemons found successfully.", pokes.len());
        Ok(pokes)
    }
}

#[async_trait::async_trait]
impl DBModel for DBPoke {
    async fn create(&mut self) -> DatabaseResult<()> {
        info!("Inserting a new pokemon to the database...");
        let db_svc = DATABASE_SERVICE.clone();
        let poke_db = db_svc
            .insert_record("pokemon", &self.id.id.to_string(), self.clone())
            .await?;

        if poke_db.is_none() {
            return Err("Failed to insert the pokemon.".into());
        }

        self.id = poke_db.unwrap().id;
        info!("Pokemon#{} inserted successfully.", self.id);
        Ok(())
    }

    // async fn update(&mut self) -> DatabaseResult<()> {
    //     info!("Updating the pokemon#{} in the database...", self.id);
    //     let db_svc = DATABASE_SERVICE.clone();
    //     self.updated_at = Utc::now();
    //     db_svc
    //         .update_record("pokemon", &self.id, self.clone())
    //
    //         .await?;

    //     info!("Pokemon#{} updated successfully.", self.id);
    //     Ok(())
    // }

    // async fn delete(&self) -> DatabaseResult<()> {
    //     info!("Deleting the pokemon#{} from the database...", self.id);

    //     let db_svc = DATABASE_SERVICE.clone();
    //     db_svc
    //         .delete_record("pokemon", &self.id)
    //
    //         .await?;

    //     info!("Pokemon#{} deleted successfully.", self.id);
    //     Ok(())
    // }
}
