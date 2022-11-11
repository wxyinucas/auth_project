use std::sync::Arc;
use axum::routing::get;
use axum::{Extension, Router};
use tera::Tera;

use page_management::extensions::InnerState;
use page_management::handlers;
use util_auth::Jwt;

#[tokio::main]

async fn main() {
    use dotenv;
    use std::env::var as config;
    dotenv::dotenv().ok();

    let jwt = Jwt::new("Rex Co.".to_string(), 3000, "42".to_string());
    let tera = Tera::new("page-management/templates/**/*.html").unwrap();
    let state = InnerState { jwt, tera };

    let app = Router::new().route(
        &*config("URL_LOGIN").unwrap(),
        get(handlers::page_login).post(handlers::login),
    )
        .layer(Extension(Arc::new(state)));

    axum::Server::bind(&config("PAGE_MANAGEMENT_ADDR").unwrap().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
