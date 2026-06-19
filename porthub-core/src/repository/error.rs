use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Workspace already exists: {name}")]
    WorkspaceAlreadyExists { name: String },

    #[error("Database error: {0}")]
    Database(#[from] libsql::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Validation error: {message}")]
    Validation { message: String },
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
