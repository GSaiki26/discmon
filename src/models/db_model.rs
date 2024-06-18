// Libs
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::DatabaseResult;

// Traits
/**
A trait to define a database model. It'll be used to define the common methods for the database models.
*/
#[async_trait]
pub trait DBModel: DeserializeOwned + Serialize + Send + Sync + Unpin {
    /**
    A method to create a new model in the database.
    */
    async fn create(&mut self) -> DatabaseResult<()>;

    // /**
    // A method to update a model in the database.
    // */
    // async fn update(&mut self) -> DatabaseResult<()>;

    // /**
    // A method to delete a model in the database.
    // */
    // async fn delete(&self) -> DatabaseResult<()>;
}
