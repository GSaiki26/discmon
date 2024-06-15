pub use cache_error::{CacheError, CacheResult};
// pub use database_error::{DatabaseError, DatabaseResult};
pub use handler_error::{HandlerError, HandlerResult};
pub use httpclient_error::{HTTPClientError, HTTPClientResult};
pub use pokeapi_error::{PokeAPIError, PokeAPIResult};

mod cache_error;
// mod database_error;
mod handler_error;
mod httpclient_error;
mod pokeapi_error;
