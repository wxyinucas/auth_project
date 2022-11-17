use svc_users::svc_users;
use util_pb::user::query_user_request::Identity;
use util_pb::user::QueryUserRequest;

#[tokio::main]
async fn main() {
    let (server, client) = svc_users().await;

    tokio::spawn(async move {
        server.await.unwrap();
        println!("here!");
    });

    let mut client = client.await.unwrap();

    let query = QueryUserRequest {
        identity: Some(Identity::Id(1)),
    };
    let res = client.query(query).await.unwrap().into_inner();
    println!("{:?}", res);
}
