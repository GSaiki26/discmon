// Libs
use super::PokeAPIError;

// Types
pub type HandlerResult<T> = Result<T, HandlerError>;

// Handler Error
#[derive(Debug)]
pub enum HandlerError {
    PokeAPIError(PokeAPIError),
    SerenityError(serenity::Error),
    Other(String),
}

impl From<PokeAPIError> for HandlerError {
    fn from(e: PokeAPIError) -> Self {
        HandlerError::PokeAPIError(e)
    }
}

impl From<serenity::Error> for HandlerError {
    fn from(e: serenity::Error) -> Self {
        HandlerError::SerenityError(e)
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
            HandlerError::PokeAPIError(e) => write!(f, "PokeAPI Error: {:?}", e),
            HandlerError::SerenityError(e) => write!(f, "Serenity Error: {:?}", e),
            HandlerError::Other(e) => write!(f, "Other Error: {:?}", e),
        }
    }
}
