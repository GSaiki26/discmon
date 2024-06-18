// Libs
use redis::RedisError;

// Types
pub type CacheResult<T> = Result<T, CacheError>;

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

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::RedisError(e) => write!(f, "Redis Error: {}", e),
            CacheError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}
