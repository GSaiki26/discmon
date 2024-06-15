// Libs
use super::{CacheError, HTTPClientError};

// Types
pub type PokeAPIResult<T> = Result<T, PokeAPIError>;

// PokeAPIError
#[derive(Debug)]
pub enum PokeAPIError {
    HTTPClientError(HTTPClientError),
    CacheError(CacheError),
    Other(String),
}

impl From<HTTPClientError> for PokeAPIError {
    fn from(error: HTTPClientError) -> Self {
        PokeAPIError::HTTPClientError(error)
    }
}

impl From<CacheError> for PokeAPIError {
    fn from(error: CacheError) -> Self {
        PokeAPIError::CacheError(error)
    }
}

impl From<String> for PokeAPIError {
    fn from(error: String) -> Self {
        PokeAPIError::Other(error)
    }
}

impl From<&str> for PokeAPIError {
    fn from(error: &str) -> Self {
        PokeAPIError::Other(error.to_string())
    }
}

impl std::fmt::Display for PokeAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokeAPIError::HTTPClientError(e) => write!(f, "HTTPClientError: {}", e),
            PokeAPIError::CacheError(e) => write!(f, "CacheError: {}", e),
            PokeAPIError::Other(e) => write!(f, "{}", e),
        }
    }
}
