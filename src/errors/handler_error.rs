// Libs
use super::{database_error::DatabaseError, PokeFinderError};

// Types
pub type HandlerResult<T> = Result<T, HandlerError>;

// Handler Error
#[derive(Debug)]
pub enum HandlerError {
    PokeFinderError(PokeFinderError),
    SerenityError(serenity::Error),
    DatabaseError(DatabaseError),
    Other(String),
}

impl From<PokeFinderError> for HandlerError {
    fn from(e: PokeFinderError) -> Self {
        HandlerError::PokeFinderError(e)
    }
}

impl From<serenity::Error> for HandlerError {
    fn from(e: serenity::Error) -> Self {
        HandlerError::SerenityError(e)
    }
}

impl From<DatabaseError> for HandlerError {
    fn from(e: DatabaseError) -> Self {
        HandlerError::DatabaseError(e)
    }
}

impl From<String> for HandlerError {
    fn from(e: String) -> Self {
        HandlerError::Other(e)
    }
}

impl From<&str> for HandlerError {
    fn from(e: &str) -> Self {
        HandlerError::Other(e.to_string())
    }
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandlerError::PokeFinderError(e) => write!(f, "PokeAPI Error: {:?}", e),
            HandlerError::DatabaseError(e) => write!(f, "Database Error: {:?}", e),
            HandlerError::SerenityError(e) => write!(f, "Serenity Error: {:?}", e),
            HandlerError::Other(e) => write!(f, "Other Error: {:?}", e),
        }
    }
}
