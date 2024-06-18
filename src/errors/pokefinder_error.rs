// Libs
use super::{CacheError, HTTPClientError};

// Types
pub type PokeFinderResult<T> = Result<T, PokeFinderError>;

// PokeFinderError
#[derive(Debug)]
pub enum PokeFinderError {
    HTTPClientError(HTTPClientError),
    CacheError(CacheError),
    Other(String),
}

impl From<HTTPClientError> for PokeFinderError {
    fn from(error: HTTPClientError) -> Self {
        PokeFinderError::HTTPClientError(error)
    }
}

impl From<CacheError> for PokeFinderError {
    fn from(error: CacheError) -> Self {
        PokeFinderError::CacheError(error)
    }
}

impl From<String> for PokeFinderError {
    fn from(error: String) -> Self {
        PokeFinderError::Other(error)
    }
}

impl From<&str> for PokeFinderError {
    fn from(error: &str) -> Self {
        PokeFinderError::Other(error.to_string())
    }
}

impl std::fmt::Display for PokeFinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokeFinderError::HTTPClientError(e) => write!(f, "HTTPClientError: {}", e),
            PokeFinderError::CacheError(e) => write!(f, "CacheError: {}", e),
            PokeFinderError::Other(e) => write!(f, "{}", e),
        }
    }
}
