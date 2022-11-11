use axum::extract::{FromRequest, RequestParts};
use axum::headers::authorization::Bearer;
use axum::headers::{Authorization, Cookie, HeaderMapExt};
use axum::TypedHeader;
use serde::de::DeserializeOwned;

use util_auth::Claims;

use crate::extensions::InnerState;
use crate::PMError;

pub struct CommonClaims<T>(pub T);

#[tonic::async_trait]
impl<B> FromRequest<B> for CommonClaims<Claims>
// TODO T or Claims？
where
    B: Send,
    // T: DeserializeOwned + 'static,
{
    type Rejection = PMError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        use dotenv;
        dotenv::dotenv().ok();

        let cookies = req.headers().typed_get::<Cookie>();
        let state = req.extensions().get::<InnerState>().unwrap();
        let cookie_name = std::env::var("TOKEN_COOKIE").expect("AUTH token name is required.");

        if let Some(cookies) = cookies {
            if let Some(token) = cookies.get(&cookie_name) {
                let claims = state
                    .jwt
                    .valid_then_get_claim(token)
                    .map_err(|_| PMError::AuthError)?;
                return Ok(CommonClaims(claims));
            }
        }

        Err(PMError::AuthError)
    }
}
