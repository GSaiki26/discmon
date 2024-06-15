// Libs

// Types
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// Database Error
#[derive(Debug)]
pub enum DatabaseError {
    DieselConnectionError(diesel::ConnectionError),
    DatabaseNotConnected,
    Other(String),
}

impl From<diesel::ConnectionError> for DatabaseError {
    fn from(error: diesel::ConnectionError) -> Self {
        DatabaseError::DieselConnectionError(error)
    }
}

impl From<String> for DatabaseError {
    fn from(error: String) -> Self {
        DatabaseError::Other(error)
    }
}

impl From<&str> for DatabaseError {
    fn from(error: &str) -> Self {
        DatabaseError::Other(error.to_string())
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::DieselConnectionError(e) => write!(f, "Diesel connection error: {}", e),
            DatabaseError::DatabaseNotConnected => write!(f, "Database not connected."),
            DatabaseError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}
