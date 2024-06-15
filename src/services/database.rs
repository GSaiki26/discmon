// Libs
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    di::database::{Database, MongoDB},
    errors::DatabaseResult,
};

// Data
pub static DATABASE_SERVICE: Lazy<Arc<DatabaseService<MongoDB>>> = Lazy::new(|| {
    let db_src = DatabaseService::new(RwLock::new(MongoDB::default()));
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
        info!("[DatabaseService] Connecting to the database...");
        self.database.write().await.connect().await?;
        info!("[DatabaseService] Connected to the database.");
        Ok(())
    }

    /**
    A method to run the database's migrations.
    */
    pub async fn migrate(&self) -> DatabaseResult<()> {
        // Run the database migrations.
        info!("[DatabaseService] Running database migrations...");
        self.database.read().await.run_migrations().await?;
        info!("[DatabaseService] Database migrations complete.");
        Ok(())
    }

    /**
    A method to get a record from the database.

    ## Parameters:
    - `tb`: The table to get the record from.
    - `id`: The ID of the record to get.
    */
    pub async fn get_record<T>(&self, tb: &str, id: &str) -> DatabaseResult<Option<T>>
    where
        T: serde::de::DeserializeOwned + Send + Sync + Unpin,
    {
        // Get the record from the database.
        info!(
            "[DatabaseService] Getting record from table '{}' with ID '{}'",
            tb, id
        );
        let record = self.database.read().await.get(tb, id).await?;
        info!(
            "[DatabaseService] Got record from table '{}' with ID '{}'",
            tb, id
        );
        Ok(record)
    }

    /**
    A method to insert a new record into the database.

    ## Parameters:
    - `tb`: The table to insert the record into.
    - `record`: The record to insert into the table.
    */
    pub async fn insert_record<T>(&self, tb: &str, record: T) -> DatabaseResult<()>
    where
        T: serde::ser::Serialize + Send + Sync + Unpin,
    {
        // Insert the record into the database.
        info!("[DatabaseService] Inserting record into table '{}'", tb);
        self.database.read().await.insert(tb, record).await?;
        info!("[DatabaseService] Inserted record into table '{}'", tb);
        Ok(())
    }

    /**
    A method to update a record in the database.

    ## Parameters:
    - `tb`: The table to update the record in.
    - `id`: The ID of the record to update.
    - `record`: The record to update the record with.
    */
    pub async fn update_record<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<()>
    where
        T: serde::ser::Serialize + Send + Sync + Unpin,
    {
        // Update the record in the database.
        info!(
            "[DatabaseService] Updating record in table '{}' with ID '{}'",
            tb, id
        );
        self.database.read().await.update(tb, id, record).await?;
        info!(
            "[DatabaseService] Updated record in table '{}' with ID '{}'",
            tb, id
        );
        Ok(())
    }

    /**
    A method to delete a record from the database.

    ## Parameters:
    - `tb`: The table to delete the record from.
    - `id`: The ID of the record to delete.
    */
    pub async fn delete_record(&self, tb: &str, id: &str) -> DatabaseResult<()> {
        // Delete the record from the database.
        info!(
            "[DatabaseService] Deleting record from table '{}' with ID '{}'",
            tb, id
        );
        self.database.read().await.delete(tb, id).await?;
        info!(
            "[DatabaseService] Deleted record from table '{}' with ID '{}'",
            tb, id
        );
        Ok(())
    }
}
