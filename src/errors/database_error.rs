// Libs

// Types
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// Database Error
#[derive(Debug)]
pub enum DatabaseError {
    SurrealDBError(surrealdb::Error),
    Other(String),
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(error: surrealdb::Error) -> Self {
        DatabaseError::SurrealDBError(error)
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
            DatabaseError::SurrealDBError(e) => write!(f, "SurrealDB error: {}", e),
            DatabaseError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}
