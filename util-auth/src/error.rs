use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, AuthError>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Jwt error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}
