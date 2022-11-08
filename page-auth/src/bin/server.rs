use std::sync::Arc;

use axum::extract::FromRequest;
use axum::headers::{Cookie, HeaderMapExt};
use axum::routing::get;
use axum::{Extension, Router};
use tera::Tera;

use page_auth::error::PageAuthError;
use page_auth::handler;
use page_auth::structs::InnerState;
use util_auth::{Claims, Jwt};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let jwt = Jwt::new("rex_secret".to_string(), 300, "Rex Wang".to_string());
    let tera = Tera::parse("page-auth/templates/*.html").unwrap();
    let inner_state = InnerState { tera, jwt };

    let app = Router::new()
        .route("/login_success", get(handler::login_success_page))
        // .layer(axum::middleware::from_extractor::<Auth>())
        .route("/login", get(handler::login_page).post(handler::log))
        .layer(Extension(Arc::new(inner_state)));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// TODO 重新放在一个地方
pub struct Auth(Claims);

/* TODO: 重新写一个
If the extractor succeeds the value will be discarded and the inner service will be called.
If the extractor fails the rejection will be returned and the inner service will not be called.

This can be used to perform validation of requests if the validation doesn’t produce any useful output,
and run the extractor for several handlers without repeating it in the function signature.

Note that if the extractor consumes the request body, as String or Bytes does,
an empty body will be left in its place. Thus wont be accessible to subsequent extractors or handlers
 */
#[tonic::async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = PageAuthError;
    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let cookies = headers.typed_get::<Cookie>();

        if let Some(cookies) = cookies {
            if let Some(token) = cookies.get("rex_token") {
                let state = req.extensions().get::<Arc<InnerState>>().unwrap();
                let claims = state
                    .jwt
                    .verify_and_get_claims(token)
                    .map_err(Self::Rejection::from)?;
                return Ok(Self(claims));
            }
        }
        Err(PageAuthError::LoginFailed)
    }
}
