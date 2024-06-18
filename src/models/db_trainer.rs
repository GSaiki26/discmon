// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::info;

use crate::{errors::DatabaseResult, services::DATABASE_SERVICE};

use super::DBModel;

// Database Trainer
/**
A struct to represent a Trainer in the database.
*/
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBTrainer {
    pub id: Thing,
    pub discord_id: String,
    pub discord_guild_id: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Implementations
impl DBTrainer {
    /**
    A method to create a new instance of DBTrainer.

    ## Parameters:
    - `discord_id`: The Discord ID of the trainer.
    - `discord_guild_id`: The Discord Guild ID of the trainer.
    */
    fn new<T>(discord_id: T, discord_guild_id: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: Thing {
                tb: String::from("trainer"),
                id: Id::ulid(),
            },
            discord_id: discord_id.into(),
            discord_guild_id: discord_guild_id.into(),
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }

    /**
    A method to find a trainer by their Discord ID.

    If the trainer is not found, a new trainer will be created.

    ## Parameters:
    - `discord_id`: The Discord ID of the trainer.
    - `discord_guild_id`: The Discord Guild ID of the trainer.
    */
    pub async fn find_by_discord_id(
        discord_id: &str,
        discord_guild_id: &str,
    ) -> DatabaseResult<Self> {
        info!("Finding the trainer by Discord ID...");
        let db_svc = DATABASE_SERVICE.clone();
        let query = format!(
            "SELECT * FROM trainer WHERE discord_id = '{}' AND discord_guild_id = '{}' LIMIT 1",
            discord_id, discord_guild_id
        );
        let mut trainer_db: Vec<Self> = db_svc.run_query(&query).await?;
        if trainer_db.is_empty() {
            info!("Trainer not found. Creating a new trainer...");
            let mut new_trainer = Self::new(discord_id, discord_guild_id);
            new_trainer.create().await?;
            return Ok(new_trainer);
        }

        info!("Trainer found successfully.");
        Ok(trainer_db.pop().unwrap())
    }
}

#[async_trait::async_trait]
impl DBModel for DBTrainer {
    async fn create(&mut self) -> DatabaseResult<()> {
        info!("Inserting a new trainer to the database...");
        let db_svc = DATABASE_SERVICE.clone();
        let trainer_db = db_svc
            .insert_record("trainer", &self.id.id.to_string(), self.clone())
            .await?;

        if trainer_db.is_none() {
            return Err("Failed to insert the trainer.".into());
        }

        self.id = trainer_db.unwrap().id;
        info!("#{} inserted successfully.", self.id);
        Ok(())
    }

    // async fn update(&mut self) -> DatabaseResult<()> {
    //     info!("Updating the trainer in the database...");
    //     let db_svc = DATABASE_SERVICE.clone();
    //     self.updated_at = Utc::now();
    //     db_svc
    //         .update_record("trainer", &self.id, self.clone())
    //
    //         .await?;

    //     info!("#{} updated successfully.", self.id);
    //     Ok(())
    // }

    // async fn delete(&self) -> DatabaseResult<()> {
    //     info!("Deleting the trainer#{} from the database...", self.id);
    //     let db_svc = DATABASE_SERVICE.clone();
    //     db_svc
    //         .delete_record("trainer", &self.id)
    //
    //         .await?;

    //     info!("#{} deleted successfully.", self.id);
    //     Ok(())
    // }
}
