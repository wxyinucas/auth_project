use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Extension;
use tera::Tera;
use tower_http::services::ServeDir;

use page_management::router::pm_router;
use page_management::State;
use util_auth::Jwt;
use util_pb::user::user_service_client::UserServiceClient;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "page_management=trace,dev_user_connect=debug");
    }
    tracing_subscriber::fmt::init();

    let jwt = Jwt::new("rex", 20000, "secret");
    let tera = Tera::new("template/**/*.html").unwrap();
    let client = UserServiceClient::connect("http://0.0.0.0:3001")
        .await
        .unwrap();
    let state = State::new(jwt, tera, Some(client));

    let app = pm_router().layer(Extension(state)).nest(
        "/assets",
        get_service(ServeDir::new("template/assets")).handle_error(|err| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("处理静态资源出错：{:?}", err),
            )
        }),
    );

    println!("working path: {:?}", std::env::current_dir());
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
