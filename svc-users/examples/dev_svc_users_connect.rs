use util_pb::user::query_user_request::Identity;
use util_pb::user::user_service_client::UserServiceClient;
use util_pb::user::QueryUserRequest;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // let addr = std::env::var("SVC_USER").unwrap();
    let addr = "http://0.0.0.0:3001";
    let mut client = UserServiceClient::connect(addr).await.unwrap();

    let query = QueryUserRequest {
        identity: Some(Identity::Id(1)),
    };
    let res = client.query(query).await.unwrap().into_inner();
    println!("{:?}", res);
}
