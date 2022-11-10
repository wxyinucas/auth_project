use thiserror::Error;

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Jwt error: {0}")]
    JwrError(#[from] jsonwebtoken::errors::Error),
}
