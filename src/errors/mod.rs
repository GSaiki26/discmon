pub use cache_error::{CacheError, CacheResult};
pub use database_error::DatabaseResult;
pub use handler_error::{HandlerError, HandlerResult};
pub use httpclient_error::{HTTPClientError, HTTPClientResult};
pub use pokefinder_error::{PokeFinderError, PokeFinderResult};

mod cache_error;
mod database_error;
mod handler_error;
mod httpclient_error;
mod pokefinder_error;
