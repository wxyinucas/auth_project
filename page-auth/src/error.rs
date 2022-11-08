use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PageAuthError {
    #[error("GREAT Tera Error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("sqlx Error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Not Implemented Error")]
    NotImplemented,

    #[error("Login Failed")]
    LoginFailed,

    #[error("Login Failed")]
    AuthError(#[from] util_auth::error::AuthError),
}

impl IntoResponse for PageAuthError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
