// Libs
use async_trait::async_trait;
use surrealdb
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::{DatabaseError, DatabaseResult};

// Database Trait
/**
A trait to represent a database. This trait is used to define the methods that a database must implement.
*/
#[async_trait]
pub trait Database {
    /**
    A method to try to establish a connection to the database.
    */
    async fn connect(&mut self) -> DatabaseResult<()>;

    /**
    A method to run the database's migrations.
    */
    async fn run_migrations(&self) -> DatabaseResult<()>;

    /**
    A method to get a record from the database.

    # Parameters:
    - `tb`: The name of the table to get the record from.
    - `id`: The ID of the record to get.
    */
    async fn get<T>(&self, tb: &str, id: &str) -> DatabaseResult<Option<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin;

    /**
    A method to get multiple records from the database.

    # Parameters:
    - `query`: The query to get the records.
    */
    async fn query<T>(&self, query: &str) -> DatabaseResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin;

    /**
    A method to insert a new record into the database.

    # Parameters:
    - `tb`: The name of the table to insert the record into.
    - `record`: The record to insert into the table.
    */
    async fn insert<T>(&self, tb: &str, record: T) -> DatabaseResult<()>
    where
        T: Serialize + Send + Sync + Unpin;

    /**
    A method to update a record in the database.

    # Parameters:
    - `tb`: The name of the table to update the record in.
    - `id`: The ID of the record to update.
    - `record`: The record to update the record with.
    */
    async fn update<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<()>
    where
        T: Serialize + Send + Sync + Unpin;

    /**
    A method to delete a record from the database.

    # Parameters:
    - `tb`: The name of the table to delete the record from.
    - `id`: The ID of the record to delete.
    */
    async fn delete(&self, tb: &str, id: &str) -> DatabaseResult<()>;
}

// Postgres
/**
A struct to represent a Postgres database.
*/
#[derive(Default)]
pub struct PostgresDatabase {
    client: Option<diesel::pg::PgConnection>,
}

#[async_trait]
impl Database for PostgresDatabase {
    async fn connect(&mut self) -> DatabaseResult<()> {
        // Connect to the database.
        let conn_url = std::env::var("DATABASE_URL").unwrap();
        let client = diesel::pg::PgConnection::establish(&conn_url)?;
        c
        self.client = Some(client);
        Ok(())
    }

    async fn run_migrations(&self) -> DatabaseResult<()> {
        // Run the migrations.
        Ok(())
    }

    async fn query<T, U>(&self, tb: &str, filter: U) -> DatabaseResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin,
        U: Send + Sync + Unpin,
    {
        // Get the client.
        let db_name = std::env::var("DATABASE_NAME").unwrap();
        let client = self
            .client
            .as_ref()
            .ok_or(DatabaseError::DatabaseNotConnected)?;

        // Get the records.
        let result = client
            .database(&db_name)
            .collection::<T>(tb)
            .find(filter, None)
            .await?
            .collect::<Vec<T>>()
            .await;

        Ok(result)
    }

    async fn get<T>(&self, tb: &str, id: &str) -> DatabaseResult<Option<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin,
    {
        // Get the client.
        let db_name = std::env::var("DATABASE_NAME").unwrap();
        let client = self
            .client
            .as_ref()
            .ok_or(DatabaseError::DatabaseNotConnected)?;

        // Get the record.
        let query = doc! {
            "_id": id.to_string()
        };
        let result = client
            .database(&db_name)
            .collection::<T>(tb)
            .find_one(query, None)
            .await?;

        Ok(result)
    }

    async fn insert<T>(&self, tb: &str, record: T) -> DatabaseResult<()>
    where
        T: Serialize + Send + Sync + Unpin,
    {
        // Get the client.
        let db_name = std::env::var("DATABASE_NAME").unwrap();
        let client = self
            .client
            .as_ref()
            .ok_or(DatabaseError::DatabaseNotConnected)?;

        // Insert the record.
        client
            .database(&db_name)
            .collection(tb)
            .insert_one(record, None)
            .await?;

        Ok(())
    }

    async fn update<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<()>
    where
        T: Serialize + Send + Sync + Unpin,
    {
        // Get the client.
        let db_name = std::env::var("DATABASE_NAME").unwrap();
        let client = self
            .client
            .as_ref()
            .ok_or(DatabaseError::DatabaseNotConnected)?;

        // Update the record.
        let record = bson::to_document(&record).unwrap();
        client
            .database(&db_name)
            .collection::<T>(tb)
            .update_one(
                doc! {
                    "_id": id.to_string()
                },
                UpdateModifications::Document(record),
                None,
            )
            .await?;

        Ok(())
    }

    async fn delete(&self, tb: &str, id: &str) -> DatabaseResult<()> {
        // Get the client.
        let db_name = std::env::var("DATABASE_NAME").unwrap();
        let client = self
            .client
            .as_ref()
            .ok_or(DatabaseError::DatabaseNotConnected)?;

        // Delete the record.
        client
            .database(&db_name)
            .collection::<Document>(tb)
            .delete_one(doc! { "_id": id.to_string() }, None)
            .await?;

        Ok(())
    }
}
