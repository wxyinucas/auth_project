
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Form, Router};
use serde::Deserialize;
use tera::Tera;

use page_management::{CommonClaims, State, UserClaims};
use util_auth::Jwt;

#[tokio::main]
async fn main() {
    let jwt = Jwt::new("rex", 20000, "secret");
    let tera = Tera::new("../../template/**/*.html").unwrap();
    let state = State::new(jwt, tera);

    let app = Router::new()
        .route("/", post(login_handler))
        .fallback(get(welcome_handler))
        .layer(Extension(state));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn welcome_handler(CommonClaims(claims): CommonClaims<UserClaims>) -> Html<String> {
    Html(String::from(format!("Welcome {claims:?}")))
}

#[axum_macros::debug_handler]
async fn login_handler(
    Extension(state): Extension<State>,
    Form(form): Form<LoginForm>,
) -> Html<String> {
    let new_claims_raw = UserClaims {
        email: form.email,
        ..UserClaims::default()
    };
    let claims = state.jwt.new_claims(new_claims_raw).unwrap();
    let token = state.jwt.token(claims).unwrap();
    Html(token)
}

#[derive(Clone, Debug, Deserialize)]
struct LoginForm {
    email: String,
}
