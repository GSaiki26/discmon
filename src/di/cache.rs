// Libs
use async_trait::async_trait;

use crate::{errors::CacheResult, utils::EnvManager};

// Cache Trait.
/**
A trait to represent a cache. This trait is used to define the methods that a cache must implement.
*/
#[async_trait]
pub trait Cache {
    type Connection;

    /**
    A method to connect to the cache server.
    */
    async fn connect(&mut self) -> CacheResult<()>;

    /**
    A method to get the key's value from the cache.

    ## Parameters:
    - `key`: The key to get the value.
    */
    async fn get_key(&self, key: &str) -> CacheResult<Option<String>>;

    /**
    A method to insert a key value pair in the cache.

    ## Parameters:
    - `key`: The key to insert.
    - `value`: The value to insert.
    */
    async fn insert_key(&self, key: &str, value: &str) -> CacheResult<()>;
}

// Redis Cache
pub mod redis_cache {
    use redis::aio::MultiplexedConnection;

    use super::*;

    /**
    A struct to represent the Redis cache.
    */
    pub struct RedisCache {
        client: redis::Client,
        conn: Option<MultiplexedConnection>,
    }

    impl RedisCache {
        /**
        A method to create a new instance of the RedisCache.
        */
        pub fn new() -> CacheResult<Self> {
            let redis_url = format!("redis://@{}", EnvManager::get_var::<String>("CACHE_HOST"));
            Ok(Self {
                client: redis::Client::open(redis_url)?,
                conn: Option::None,
            })
        }
    }

    #[async_trait]
    impl Cache for RedisCache {
        type Connection = MultiplexedConnection;

        async fn connect(&mut self) -> CacheResult<()> {
            self.conn = Some(self.client.get_multiplexed_tokio_connection().await?);
            Ok(())
        }

        async fn get_key(&self, key: &str) -> CacheResult<Option<String>> {
            let mut conn = match self.conn.clone() {
                Some(conn) => conn,
                None => return Err("No connection to the Redis server.".into()),
            };
            let key = format!(
                "{}:{}",
                EnvManager::get_var::<String>("CACHE_NAMESPACE"),
                key
            );
            Ok(redis::cmd("GET").arg(key).query_async(&mut conn).await?)
        }

        async fn insert_key(&self, key: &str, value: &str) -> CacheResult<()> {
            let mut conn = match self.conn.clone() {
                Some(conn) => conn,
                None => return Err("No connection to the Redis server.".into()),
            };
            let key = format!(
                "{}:{}",
                EnvManager::get_var::<String>("CACHE_NAMESPACE"),
                key
            );
            Ok(redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query_async(&mut conn)
                .await?)
        }
    }
}
