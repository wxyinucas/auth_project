use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, JwtError>;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Jwt error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}
