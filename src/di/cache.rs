// Libs
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;

use crate::errors::CacheResult;

// Cache Trait.
#[async_trait]
pub trait Cache {
    type Connection;

    /**
    A method to connect to the cache server.
    */
    async fn connect(&mut self) -> CacheResult<()>;

    /**
    A method to check if the cache is connected.
    */
    fn is_connected(&self) -> bool;

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
pub struct RedisCache {
    client: redis::Client,
    conn: Option<MultiplexedConnection>,
}

impl RedisCache {
    /**
    A method to create a new instance of the RedisCache.
    */
    pub fn new() -> CacheResult<Self> {
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

    async fn connect(&mut self) -> CacheResult<()> {
        self.conn = Some(self.client.get_multiplexed_tokio_connection().await?);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.conn.is_some()
    }

    async fn get_key(&self, key: &str) -> CacheResult<Option<String>> {
        let mut conn = match self.conn.clone() {
            Some(conn) => conn,
            None => return Err("No connection to the Redis server.".into()),
        };
        Ok(redis::cmd("GET").arg(key).query_async(&mut conn).await?)
    }

    async fn insert_key(&self, key: &str, value: &str) -> CacheResult<()> {
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
