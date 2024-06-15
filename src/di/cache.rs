// Libs
use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, RedisError};
use tracing::info;

// Cache Error
#[derive(Debug)]
pub enum CacheError {
    RedisError(RedisError),
    Other(String),
}

impl From<RedisError> for CacheError {
    fn from(error: RedisError) -> Self {
        CacheError::RedisError(error)
    }
}

impl From<String> for CacheError {
    fn from(error: String) -> Self {
        CacheError::Other(error)
    }
}

impl From<&str> for CacheError {
    fn from(error: &str) -> Self {
        CacheError::Other(error.to_string())
    }
}

// Cache Trait.
#[async_trait]
pub trait Cache {
    type Connection;

    /**
    A method to connect to the cache server.
    */
    async fn connect(&mut self) -> Result<(), CacheError>;

    /**
    A method to check if the cache is connected.
    */
    fn is_connected(&self) -> bool;

    /**
    A method to get the key's value from the cache.

    ## Parameters:
    - `key`: The key to get the value.
    */
    async fn get_key(&self, key: &str) -> Result<Option<String>, CacheError>;

    /**
    A method to insert a key value pair in the cache.

    ## Parameters:
    - `key`: The key to insert.
    - `value`: The value to insert.
    */
    async fn insert_key(&self, key: &str, value: &str) -> Result<(), CacheError>;
}

// Redis Cache
pub struct RedisCache {
    client: redis::Client,
    conn: Option<MultiplexedConnection>,
}

impl RedisCache {
    /**
    A method to create a new instance of the RedisCache.
    */
    pub fn new() -> Result<Self, CacheError> {
        let redis_url = format!(
            "redis://@{}:{}",
            std::env::var("REDIS_HOST").unwrap(),
            std::env::var("REDIS_PORT").unwrap(),
            // std::env::var("REDIS_USER").unwrap(),
            // std::env::var("REDIS_PASS").unwrap(),
        );

        Ok(Self {
            client: redis::Client::open(redis_url)?,
            conn: Option::None,
        })
    }
}

#[async_trait]
impl Cache for RedisCache {
    type Connection = MultiplexedConnection;

    async fn connect(&mut self) -> Result<(), CacheError> {
        info!("[CACHE] Connecting to the Redis server...");
        self.conn = Some(self.client.get_multiplexed_tokio_connection().await?);
        info!("[CACHE] Connected to the Redis server.");
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.conn.is_some()
    }

    async fn get_key(&self, key: &str) -> Result<Option<String>, CacheError> {
        let mut conn = match self.conn.clone() {
            Some(conn) => conn,
            None => return Err("No connection to the Redis server.".into()),
        };
        Ok(redis::cmd("GET").arg(key).query_async(&mut conn).await?)
    }

    async fn insert_key(&self, key: &str, value: &str) -> Result<(), CacheError> {
        let mut conn = match self.conn.clone() {
            Some(conn) => conn,
            None => return Err("No connection to the Redis server.".into()),
        };
        Ok(redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut conn)
            .await?)
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use mockall::mock;
//     // use std::sync::Arc;
//     // use tokio::sync::Mutex;

//     mock! {
//         pub Cache {}
//         #[async_trait]
//         impl Cache for Cache {
//             type Connection = ();
//             async fn connect(&mut self) -> Result<(), CacheError>;
//             async fn get_key(&self, key: &str) -> Result<Option<String>, CacheError>;
//             async fn insert_key(&self, key: &str, value: &str) -> Result<(), CacheError>;
//         }
//     }

// #[derive(Default)]
// pub struct CacheMock {
//     pub connect_calls: Arc<Mutex<Vec<()>>>,
//     pub get_key_calls: Arc<Mutex<Vec<String>>>,
//     pub insert_key_calls: Arc<Mutex<Vec<(String, String)>>>,

//     pub get_key_return: Option<String>,
// }

// #[async_trait]
// impl Cache for CacheMock {
//     type Connection = ();

//     async fn connect(&mut self) -> Result<(), CacheError> {
//         self.connect_calls.lock().await.push(());
//         Ok(())
//     }

//     async fn get_key(&self, _key: &str) -> Result<Option<String>, CacheError> {
//         self.get_key_calls.lock().await.push(_key.to_string());
//         Ok(self.get_key_return.clone())
//     }

//     async fn insert_key(&self, _key: &str, _value: &str) -> Result<(), CacheError> {
//         self.insert_key_calls
//             .lock()
//             .await
//             .push((_key.to_string(), _value.to_string()));
//         Ok(())
//     }
// }
// }
