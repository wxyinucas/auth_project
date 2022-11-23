use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(index));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    "hello world!".to_string()
}
