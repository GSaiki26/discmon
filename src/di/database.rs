// Libs
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::DatabaseResult;

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
    A method to get multiple records from the database.

    If the query returns multiple records, it'll return the results from the first query.

    # Parameters:
    - `query`: The query to get the records.
    */
    async fn query<T>(&self, query: &str) -> DatabaseResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync + Unpin;

    // /**
    // A method to get a record from the database.

    // # Parameters:
    // - `tb`: The name of the table to get the record from.
    // - `id`: The ID of the record to get.
    // */
    // async fn get<T>(&self, tb: &str, id: &Ulid) -> DatabaseResult<Option<T>>
    // where
    //     T: DeserializeOwned + Send + Sync + Unpin;

    /**
    A method to insert a new record into the database.

    # Parameters:
    - `tb`: The name of the table to insert the record into.
    - `record`: The record to insert into the table.
    */
    async fn insert<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<Option<T>>
    where
        T: DeserializeOwned + Serialize + Send + Sync + Unpin;

    // /**
    // A method to update a record in the database.

    // # Parameters:
    // - `tb`: The name of the table to update the record in.
    // - `id`: The ID of the record to update.
    // - `record`: The record to update the record with.
    // */
    // async fn update<T>(&self, tb: &str, id: &Id, record: T) -> DatabaseResult<()>
    // where
    //     T: Serialize + Send + Sync + Unpin;

    // /**
    // A method to delete a record from the database.

    // # Parameters:
    // - `tb`: The name of the table to delete the record from.
    // - `id`: The ID of the record to delete.
    // */
    // async fn delete(&self, tb: &str, id: &Id) -> DatabaseResult<()>;
}

// SurrealDB
pub mod surreal_db {
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth,
        sql::{Id, Thing},
        Surreal,
    };

    use super::*;
    use crate::utils::EnvManager;

    /**
    A struct to represent a SurrealDB database.
    */
    #[derive(Default)]
    pub struct SurrealDB {
        conn: Option<Surreal<Client>>,
    }

    #[async_trait]
    impl Database for SurrealDB {
        async fn connect(&mut self) -> DatabaseResult<()> {
            // Connect to the database.
            let conn_url: String = EnvManager::get_var("DATABASE_HOST");
            let client = surrealdb::Surreal::new::<Ws>(&conn_url).await?;

            client
                .signin(auth::Root {
                    username: &EnvManager::get_var::<String>("DATABASE_USER"),
                    password: &EnvManager::get_var::<String>("DATABASE_PASS"),
                })
                .await?;

            client
                .use_ns(&EnvManager::get_var::<String>("DATABASE_NAMESPACE"))
                .use_db(&EnvManager::get_var::<String>("DATABASE_NAME"))
                .await?;

            self.conn = Some(client);
            Ok(())
        }

        async fn run_migrations(&self) -> DatabaseResult<()> {
            let conn = self.conn.clone().unwrap();

            conn.query("DEFINE TABLE trainer SCHEMAFULL").await?;
            conn.query("DEFINE FIELD discord_id ON TABLE trainer TYPE string")
                .await?;
            conn.query("DEFINE FIELD discord_guild_id ON TABLE trainer TYPE string")
                .await?;
            conn.query("DEFINE FIELD created_at ON TABLE trainer TYPE datetime")
                .await?;
            conn.query("DEFINE FIELD updated_at ON TABLE trainer TYPE datetime")
                .await?;

            conn.query("DEFINE TABLE pokemon SCHEMAFULL").await?;
            conn.query("DEFINE FIELD trainer_id ON TABLE pokemon TYPE record")
                .await?;
            conn.query("DEFINE FIELD poke_id ON TABLE pokemon TYPE number")
                .await?;
            conn.query("DEFINE FIELD is_shiny ON TABLE pokemon TYPE bool")
                .await?;
            conn.query("DEFINE FIELD created_at ON TABLE pokemon TYPE datetime")
                .await?;
            conn.query("DEFINE FIELD updated_at ON TABLE pokemon TYPE datetime")
                .await?;

            Ok(())
        }

        async fn query<T>(&self, query: &str) -> DatabaseResult<Vec<T>>
        where
            T: DeserializeOwned + Send + Sync + Unpin,
        {
            let mut response = self.conn.as_ref().unwrap().query(query).await?;
            Ok(response.take(0)?)
        }

        // async fn get<T>(&self, tb: &str, id: &Ulid) -> DatabaseResult<Option<T>>
        // where
        //     T: DeserializeOwned + Send + Sync + Unpin,
        // {
        //     // Get the record.
        //     let locator = (tb, id.to_string());
        //     let conn = self.conn.as_ref().unwrap();
        //     Ok(conn.select(locator).await?)
        // }

        async fn insert<T>(&self, tb: &str, id: &str, record: T) -> DatabaseResult<Option<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync + Unpin,
        {
            // Insert the record.
            let locator = Thing {
                tb: tb.to_string(),
                id: Id::from(id.to_string()),
            };
            let conn = self.conn.as_ref().unwrap();
            Ok(conn.insert::<Option<T>>(locator).content(record).await?)
        }

        // async fn update<T>(&self, tb: &str, id: &Ulid, record: T) -> DatabaseResult<()>
        // where
        //     T: Serialize + Send + Sync + Unpin,
        // {
        //     // Update the record.
        //     let locator = (tb, id.to_string());
        //     let conn = self.conn.as_ref().unwrap();
        //     conn.update::<Option<()>>(locator).content(record).await?;
        //     Ok(())
        // }

        // async fn delete(&self, tb: &str, id: &Ulid) -> DatabaseResult<()> {
        //     // Delete the record.
        //     let locator = (tb, id.to_string());
        //     let conn = self.conn.as_ref().unwrap();
        //     conn.delete::<Option<()>>(locator).await?;
        //     Ok(())
        // }
    }
}
