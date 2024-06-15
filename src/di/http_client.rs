// Libs
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::errors::{HTTPClientError, HTTPClientResult};

// Cache Trait.
#[async_trait]
pub trait HTTPClient {
    /**
    A method to connect to a HTTP server.
    */
    async fn access<T>(&self, method: &str, url: &str) -> HTTPClientResult<T>
    where
        T: DeserializeOwned;
}

// Reqwest HTTP Client

pub struct ReqwestHTTPClient;

impl ReqwestHTTPClient {
    /**
    A method to create a new instance of the ReqwestHTTPClient.
    */
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HTTPClient for ReqwestHTTPClient {
    async fn access<T>(&self, method: &str, url: &str) -> HTTPClientResult<T>
    where
        T: DeserializeOwned,
    {
        // Create a new reqwest client.
        debug!("Making a request {} {}", method, url);
        let client = reqwest::Client::new();

        // Send the request.
        let response = match method.to_uppercase().as_str() {
            "GET" => client.get(url).send().await?,
            "POST" => client.post(url).send().await?,
            "PUT" => client.put(url).send().await?,
            "DELETE" => client.delete(url).send().await?,
            _ => return Err(HTTPClientError::MethodNotSupported),
        };

        // Parse the response.
        debug!("Parsing the response");
        Ok(response.json::<T>().await?)
    }
}
