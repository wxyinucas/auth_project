use thiserror::Error;

#[derive(Debug, Error)]
pub enum PageAuthError {
    #[error("GREAT Tera Error: {0}")]
    TeraError(#[from] tera::Error),
}

