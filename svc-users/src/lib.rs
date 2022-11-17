use crate::db_pool::UserDBPool;
use crate::service::UserInnerService;
use sqlx::PgPool;
use std::future::Future;
use tonic::transport::{Channel, Error};
use util_pb::user::user_service_client::UserServiceClient;
use util_pb::user::user_service_server::UserServiceServer;

pub mod db_pool;
pub mod error;
pub mod service;
pub mod traits;

pub async fn svc_users() -> (
    impl Future<Output = Result<(), Error>>,
    impl Future<Output = Result<UserServiceClient<Channel>, Error>>,
) {
    dotenv::dotenv().ok();
    let addr = std::env::var("SVC_USER").unwrap();
    let db_addr = std::env::var("DATABASE_URL").unwrap();

    tracing::info!("SVC_USER listening on: {}", addr);
    println!("SVC_USER listening on: {}", addr);
    let pg_pool = PgPool::connect(&db_addr).await.unwrap();
    let inner_svc = UserInnerService::new(UserDBPool { pool: pg_pool });
    let svc = UserServiceServer::new(inner_svc);

    let server = tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr.parse().unwrap());

    let addr = "http://0.0.0.0:3001";
    let client = UserServiceClient::connect(addr);
    (server, client)
}
