#![allow(clippy::all, unused_imports, dead_code)]

use sqlx_db_tester::TestDb;

use svc_users::db_pool::UserDBPool;
use svc_users::error::Result;
use svc_users::UserDB;
use util_pb::user::{query_user_request, CreateUserRequest};

#[tokio::main]
async fn main() {
    println!("{:?}", std::env::current_dir());
    let tdb = TestDb::new("localhost", 5432, "", "", "./migrations");
    let pool = tdb.get_pool().await;
    let user_db_pool = UserDBPool::new(pool);

    // insert
    let user = CreateUserRequest {
        email: "rex@mail.com".to_string(),
        password: "rex_pwd".to_string(),
    };

    let id = user_db_pool.insert(&user).await.unwrap();
    assert_eq!(id, 1);

    // query
    let identity1 = query_user_request::Identity::Id(id);
    let user1 = user_db_pool
        .query(identity1.clone())
        .await
        .unwrap()
        .unwrap();

    let identity2 = query_user_request::Identity::Email("rex@mail.com".to_string());
    let user2 = user_db_pool.query(identity2).await.unwrap().unwrap();

    assert_eq!(user1, user2);

    // delete
    let user3 = user_db_pool.delete(id).await.unwrap();
    assert_eq!(user1, user3);

    let res = user_db_pool.query(identity1).await.unwrap();
    assert!(res.is_none());
}

pub(crate) type UserId = i32;
