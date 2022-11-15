use thiserror::Error;

pub type Result<T> = std::result::Result<T, UsersError>;
#[derive(Error, Debug)]
pub enum UsersError {
    #[error("Sqlx Error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
