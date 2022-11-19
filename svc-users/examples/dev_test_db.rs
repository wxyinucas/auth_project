#![allow(clippy::all, unused_imports, dead_code)]

use std::path::Path;

use dotenv::dotenv;
use sqlx_db_tester::TestPg;

use svc_users::db_pool::UserDBPool;
use svc_users::error::Result;
use svc_users::traits::UserDB;
use util_pb::user::{CreateUserRequest, QueryUserRequest};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("{:?}", std::env::current_dir());
    // TestPg::new("postgres://localhost:5432".to_string(), Path::new("."));  // 记录了新的格式
    let tdb = TestPg::new(std::env::var("TDB_URL").unwrap(), Path::new("./migrations")); // TODO test 从环境中读信息
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
    let identity1 = QueryUserRequest {
        id,
        ..QueryUserRequest::default()
    };
    let user1 = user_db_pool.query(identity1.clone()).await.unwrap();

    let identity2 = QueryUserRequest {
        email: "rex@mail.com".to_string(),
        ..QueryUserRequest::default()
    };
    let user2 = user_db_pool.query(identity2).await.unwrap();

    assert_eq!(user1, user2);

    // delete
    let user3 = user_db_pool.delete(id).await.unwrap();
    assert_eq!(user1[0], user3);

    let res = user_db_pool.query(identity1).await.unwrap();
    assert!(res.is_empty());
}

pub(crate) type UserId = i32;
