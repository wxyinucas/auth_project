use jsonwebtoken as jwt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Jwt Error: {0}")]
    JwtError(#[from] jwt::errors::Error),


}
