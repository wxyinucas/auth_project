use axum::extract::rejection::TypedHeaderRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, PMError>;

#[derive(Error, Debug)]
pub enum PMError {
    #[error("Unauthorized")]
    AuthError,

    #[error("Internal: {0}")]
    InternalError(#[from] TypedHeaderRejection),
}

impl IntoResponse for PMError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            PMError::AuthError => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            PMError::InternalError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        };
        (code, msg).into_response()
    }
}
