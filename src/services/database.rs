// Libs
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::di::{Database, SurrealDB};
use crate::errors::DatabaseResult;

// Data
pub static DATABASE_SERVICE: Lazy<Arc<DatabaseService<SurrealDB>>> = Lazy::new(|| {
    let db_src = DatabaseService::new(RwLock::new(SurrealDB::default()));
    Arc::new(db_src)
});

// Database Service
pub struct DatabaseService<DB>
where
    DB: Database,
{
    database: RwLock<DB>,
}

impl<DB> DatabaseService<DB>
where
    DB: Database,
{
    /**
    A method to create a new DatabaseService instance.

    ## Parameters:
    - `database`: The database to use for the service.
    */
    pub fn new(database: RwLock<DB>) -> Self {
        DatabaseService { database }
    }

    /**
    A method to try to establish a connection to the database.
    */
    pub async fn connect(&self) -> DatabaseResult<()> {
        // Connect to the database.
        info!("Connecting to the database...");
        self.database.write().await.connect().await?;
        info!("Connected to the database.");
        Ok(())
    }

    /**
    A method to run the database's migrations.
    */
    pub async fn run_migrations(&self) -> DatabaseResult<()> {
        // Run the database migrations.
        info!("Running database migrations...");
        self.database.read().await.run_migrations().await?;
        info!("Database migrations complete.");
        Ok(())
    }

    /**
    A method to run a database query.

    ## Parameters:
    - `query`: The query to run.
    */
    pub async fn run_query<T>(&self, query: &str) -> DatabaseResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin,
    {
        // Run the query on the database.
        debug!("Running query: '{}'", query);
        let result = self.database.read().await.query(query).await?;
        info!("Query ran successfully.");
        Ok(result)
    }

    // /**
    // A method to get a record from the database.

    // ## Parameters:
    // - `tb`: The table to get the record from.
    // - `id`: The ID of the record to get.
    // */
    // pub async fn get_record<T>(&self, tb: &str, id: &Ulid) -> DatabaseResult<Option<T>>
    // where
    //     T: DeserializeOwned + Send + Sync + Unpin,
    // {
    //     // Get the record from the database.
    //     info!("Getting record from table '{}' with ID '{}'", tb, id);
    //     let record = self.database.read().await.get(tb, id).await?;
    //     info!("Got record from table '{}' with ID '{}'", tb, id);
    //     Ok(record)
    // }

    /**
    A method to insert a new record into the database.

    ## Parameters:
    - `tb`: The table to insert the record into.
    - `record`: The record to insert into the table.
    */
    pub async fn insert_record<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<Option<T>>
    where
        T: DeserializeOwned + std::fmt::Debug + Serialize + Send + Sync + Unpin,
    {
        // Insert the record into the database.
        info!("Inserting record #{}:{}...", tb, id);
        let result = self.database.read().await.insert(tb, id, record).await?;
        info!("Inserted record #{}:{}.", tb, id);
        Ok(result)
    }

    // /**
    // A method to update a record in the database.

    // ## Parameters:
    // - `tb`: The table to update the record in.
    // - `id`: The ID of the record to update.
    // - `record`: The record to update the record with.
    // */
    // pub async fn update_record<T>(&self, tb: &str, id: &Ulid, record: T) -> DatabaseResult<()>
    // where
    //     T: Serialize + Send + Sync + Unpin,
    // {
    //     // Update the record in the database.
    //     info!("Updating record in table '{}' with ID '{}'", tb, id);
    //     self.database.read().await.update(tb, id, record).await?;
    //     info!("Updated record in table '{}' with ID '{}'", tb, id);
    //     Ok(())
    // }

    // /**
    // A method to delete a record from the database.

    // ## Parameters:
    // - `tb`: The table to delete the record from.
    // - `id`: The ID of the record to delete.
    // */
    // pub async fn delete_record(&self, tb: &str, id: &Ulid) -> DatabaseResult<()> {
    //     // Delete the record from the database.
    //     info!("Deleting record from table '{}' with ID '{}'", tb, id);
    //     self.database.read().await.delete(tb, id).await?;
    //     info!("Deleted record from table '{}' with ID '{}'", tb, id);
    //     Ok(())
    // }
}
