// Libs

// Types
pub type HTTPClientResult<T> = Result<T, HTTPClientError>;

// HTTPClient Error
#[derive(Debug)]
pub enum HTTPClientError {
    Reqwest(reqwest::Error),
    MethodNotSupported,
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

impl std::fmt::Display for HTTPClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPClientError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            HTTPClientError::MethodNotSupported => write!(f, "Method not supported."),
            HTTPClientError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}
