use std::ops::Deref;

use sqlx::{PgPool, Row};

use util_pb::user::{CreateUserRequest, QueryUserRequest, User};
use util_pb::ToSql;

use crate::{
    error::{Result, UsersError},
    traits::UserDB,
    UserId,
};

pub struct UserDBPool {
    pub pool: PgPool,
}

impl UserDBPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Deref for UserDBPool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

#[tonic::async_trait]
impl UserDB for UserDBPool {
    async fn query(&self, req: QueryUserRequest) -> Result<Vec<User>> {
        let query = req.to_sql();

        let row = sqlx::query_as(&query)
            .fetch_all(self.deref())
            .await
            .map_err(UsersError::from)?;
        Ok(row)
    }

    async fn insert(&self, user: &CreateUserRequest) -> Result<UserId> {
        let sql = "INSERT INTO auth.users (email, password) VALUES ($1, $2) RETURNING id";
        let res = sqlx::query(sql)
            .bind(&user.email)
            .bind(&user.password)
            .fetch_one(self.deref())
            .await
            .map_err(UsersError::from)?
            .get(0);
        Ok(res)
    }

    async fn delete(&self, id: UserId) -> Result<User> {
        let sql = "DELETE FROM auth.users WHERE id = $1 RETURNING *";
        let res = sqlx::query_as(sql)
            .bind(id)
            .fetch_one(self.deref())
            .await
            .map_err(UsersError::from)?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use dotenv::dotenv;
    use sqlx_db_tester::TestPg;

    use util_pb::user::{CreateUserRequest, QueryUserRequest};

    use crate::db_pool::UserDBPool;
    use crate::traits::UserDB;

    #[tokio::test]
    async fn user_db_pool_should_work() {
        dotenv().ok();
        println!("{:?}", std::env::current_dir());
        let tdb = TestPg::new(
            std::env::var("TDB_URL").unwrap(),
            Path::new("../migrations"),
        ); // TODO test 从环境中读信息
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
}
