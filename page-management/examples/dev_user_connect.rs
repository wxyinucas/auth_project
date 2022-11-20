#![allow(clippy::all, unused_imports, dead_code)]

use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, get_service, post};
use axum::{Extension, Form, Router};
use serde::Deserialize;
use tera::Tera;
use tower_http::services::ServeDir;

use page_management::handlers::{handler_login, page_dashboard, page_login, page_user_index};
use page_management::router::pm_router;
use page_management::{CommonClaims, State, UserClaims, DASHBOARD, LOGIN, USER_INDEX};
use util_auth::Jwt;
use util_pb::user::user_service_client::UserServiceClient;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "page_management=trace,dev_user_connect=debug");
    }
    tracing_subscriber::fmt::init();

    let jwt = Jwt::new("rex", 20000, "secret");
    let tera = Tera::new("page-management/template/**/*.html").unwrap();
    let client = UserServiceClient::connect("http://0.0.0.0:3001")
        .await
        .unwrap();
    let state = State::new(jwt, tera, Some(client));

    let app = pm_router().layer(Extension(state)).nest(
        "/assets",
        get_service(ServeDir::new("page-management/template/assets")).handle_error(
            |err| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("处理静态资源出错：{:?}", err),
                )
            },
        ),
    );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
