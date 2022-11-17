use sqlx::PgPool;

use svc_users::db_pool::UserDBPool;
use svc_users::service::UserInnerService;
use util_pb::user::user_service_server::UserServiceServer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let addr = std::env::var("SVC_USER").unwrap();
    let db_addr = std::env::var("DATABASE_URL").unwrap();

    tracing::info!("SVC_USER listening on: {}", addr);
    println!("SVC_USER listening on: {}", addr);
    let pg_pool = PgPool::connect(&db_addr).await.unwrap();
    let inner_svc = UserInnerService::new(UserDBPool { pool: pg_pool });
    let svc = UserServiceServer::new(inner_svc);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
