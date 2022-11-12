use axum::middleware::from_extractor;
use axum::routing::get;
use axum::{Extension, Router};
use tera::Tera;

use page_management::extensions::{InnerState, State};
use page_management::handlers;
use page_management::middleware::CommonClaims;
use util_auth::{Claims, Jwt};

#[tokio::main]

async fn main() {
    use dotenv;
    use std::env::var as config;
    dotenv::dotenv().ok();

    let jwt = Jwt::new("Rex Co.".to_string(), 3000, "42".to_string());
    let tera = Tera::new("page-management/templates/**/*.html").unwrap();
    let state = State::new(InnerState { jwt, tera });

    let app = Router::new()
        .route(
            &config("URL_DASHBOARD").unwrap(),
            get(handlers::page_dashboard),
        )
        .layer(from_extractor::<CommonClaims<Claims>>())
        .route(
            &config("URL_LOGIN").unwrap(),
            get(handlers::page_login).post(handlers::login),
        )
        .layer(Extension(state));

    axum::Server::bind(&config("PAGE_MANAGEMENT_ADDR").unwrap().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
