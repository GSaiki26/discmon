// Libs
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tracing::debug;

// Cache Error
#[derive(Debug)]
pub enum HTTPClientError {
    Reqwest(reqwest::Error),
    Other(String),
}

impl From<reqwest::Error> for HTTPClientError {
    fn from(error: reqwest::Error) -> Self {
        HTTPClientError::Reqwest(error)
    }
}

impl From<String> for HTTPClientError {
    fn from(error: String) -> Self {
        HTTPClientError::Other(error)
    }
}

impl From<&str> for HTTPClientError {
    fn from(error: &str) -> Self {
        HTTPClientError::Other(error.to_string())
    }
}

// Cache Trait.
#[async_trait]
pub trait HTTPClient {
    /**
    A method to connect to a HTTP server.
    */
    async fn access<T>(&self, method: &str, url: &str) -> Result<T, HTTPClientError>
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
    async fn access<T>(&self, method: &str, url: &str) -> Result<T, HTTPClientError>
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
            _ => return Err(HTTPClientError::Other("Method not supported".to_string())),
        };

        // Parse the response.
        debug!("Parsing the response");
        Ok(response.json::<T>().await?)
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use mockall::mock;
//     // use std::sync::Arc;
//     // use tokio::sync::Mutex;

//     mock! {
//         pub HTTPClient {}
//         #[async_trait]
//         impl HTTPClient for HTTPClient {
//             async fn access<T>(&self, method: &str, url: &str) -> Result<T, HTTPClientError>
//             where
//                 T: DeserializeOwned + 'static;
//         }
//     }

// #[derive(Default)]
// pub struct HTTPClientMock {
//     pub access_calls: Arc<Mutex<Vec<(String, String)>>>,
//     pub access_return: String,
// }

// #[async_trait]
// impl HTTPClient for HTTPClientMock {
//     async fn access<T>(&self, method: &str, url: &str) -> Result<T, HTTPClientError>
//     where
//         T: DeserializeOwned,
//     {
//         self.access_calls
//             .lock()
//             .await
//             .push((method.to_string(), url.to_string()));
//         Ok(serde_json::from_str(&self.access_return).unwrap())
//     }
// }
// }
