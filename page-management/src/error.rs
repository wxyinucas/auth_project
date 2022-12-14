use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, PMError>;

#[derive(Error, Debug)]
pub enum PMError {
    #[error("Jwt Error: {0}")]
    JwtError(#[from] util_auth::JwtError),

    #[error("Auth Error")]
    AuthError,

    #[error("Tera Error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("Lack client of: {0}")]
    LackClientError(&'static str),

    #[error("Inner Svc Error: {0}")]
    InnerSvcError(String),

    #[error("Not implement Error: {0}")]
    NotImplementedError(&'static str),
}

impl IntoResponse for PMError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
