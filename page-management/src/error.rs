use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PMError {
    #[error("Jwt Error: {0}")]
    JwtError(#[from] util_auth::JwtError),

    #[error("Auth Error")]
    AuthError,
}

impl IntoResponse for PMError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
