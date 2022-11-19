#![allow(clippy::all, unused_imports, dead_code)]
use svc_users::svc_users;
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
        id: 1,
        ..QueryUserRequest::default()
    };
    let res = client.query(query).await.unwrap().into_inner();
    println!("{:?}", res);
}
