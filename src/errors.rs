use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Database error: [0")]
    Db(#[from] rusqlite::Error), // #[from] auto converts error into TodoError::Db
    #[error("Task with id {0} not found")]
    NotFoundTaskId(i64),
}