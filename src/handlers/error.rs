// Libs
use crate::services::pokeapi::PokeAPIError;

// HandlerError
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
