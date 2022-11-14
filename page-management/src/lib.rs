use std::ops::Deref;
use std::sync::Arc;

use axum::extract::{FromRequest, RequestParts};
use axum::headers::{Cookie, HeaderMapExt};
use serde::{Deserialize, Serialize};
use tera::Tera;

use util_auth::claims::Claims;
use util_auth::Jwt;

use crate::error::PMError;

pub mod error;

const LOGIN: &'static str = "/login";
const DASHBOARD: &'static str = "/dashboard";
const TOKEN_COOKIE: &'static str = "rex_token";

/* =================================================================


 Extension


================================================================== */

pub struct InnerState {
    pub jwt: Jwt,
    pub tera: Tera,
}

pub struct State {
    pub inner: Arc<InnerState>,
}

impl State {
    pub fn new(jwt: Jwt, tera: Tera) -> Self {
        let inner = InnerState { jwt, tera };
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl Deref for State {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/* =================================================================


 Extractor


================================================================== */
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserClaims {
    pub iss: String,
    pub exp: usize,
    pub email: String,
}

impl Claims for UserClaims {
    fn iss(self, iss: &str) -> Self {
        Self {
            iss: iss.to_string(),
            ..self
        }
    }

    fn exp(self, exp: usize) -> Self {
        Self { exp, ..self }
    }
}

pub struct CommonClaims<T: Claims>(pub T);

impl<T: Claims> CommonClaims<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[tonic::async_trait]
impl<T, B> FromRequest<B> for CommonClaims<T>
where
    B: Send,
    T: Claims + 'static,
{
    type Rejection = PMError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = req
            .headers()
            .typed_get::<Cookie>()
            .ok_or(PMError::AuthError)?;
        let token = cookie.get(TOKEN_COOKIE).ok_or(PMError::AuthError)?;
        let state = req.extensions().get::<State>().unwrap();

        let claims = state
            .jwt
            .validate_and_get_claims::<T>(token)
            .map_err(PMError::from)?;

        Ok(CommonClaims(claims))
    }
}
