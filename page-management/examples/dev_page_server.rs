use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, get_service, post};
use axum::{Extension, Form, Router};
use serde::Deserialize;
use tera::Tera;
use tower_http::services::ServeDir;

use page_management::handlers::{handler_login, page_dashboard, page_login};
use page_management::{CommonClaims, State, UserClaims, DASHBOARD, LOGIN};
use util_auth::Jwt;

#[tokio::main]
async fn main() {
    let jwt = Jwt::new("rex", 20000, "secret");
    let tera = Tera::new("page-management/template/**/*.html").unwrap();
    let state = State::new(jwt, tera);

    let app = Router::new()
        .route(LOGIN, get(page_login).post(handler_login))
        .route(DASHBOARD, get(page_dashboard))
        .layer(Extension(state))
        .nest(
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

async fn welcome_handler(CommonClaims(claims): CommonClaims<UserClaims>) -> Html<String> {
    Html(String::from(format!("Welcome {claims:?}")))
}
