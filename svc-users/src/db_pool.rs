use std::ops::Deref;

use sqlx::{PgPool, Row};

use util_pb::user::{query_user_request, User};

use crate::{
    error::{Result, UsersError},
    UserDB, UserId,
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
    async fn query(&self, identity: query_user_request::Identity) -> Result<Option<User>> {
        let query = match identity {
            query_user_request::Identity::Id(id) => {
                let sql = "SELECT * FROM auth.users WHERE id = $1";
                sqlx::query_as(sql).bind(id)
            }
            query_user_request::Identity::Email(email) => {
                let sql = "SELECT * FROM auth.users WHERE email = $1";
                sqlx::query_as(sql).bind(email)
            }
        };

        let row = query
            .fetch_optional(self.deref())
            .await
            .map_err(UsersError::from)?;
        Ok(row)
    }

    async fn insert(&self, user: &User) -> Result<UserId> {
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
    use sqlx_db_tester::TestDb;

    use util_pb::user::{query_user_request, User};

    use crate::db_pool::UserDBPool;
    use crate::UserDB;

    #[tokio::test]
    async fn pg_db_should_work() {
        println!("{:?}", std::env::current_dir());
        let tdb = TestDb::new("localhost", 5432, "", "", "../migrations");
        let pool = tdb.get_pool().await;
        let user_db_pool = UserDBPool::new(pool);

        // insert
        let user = User {
            id: 0,
            email: "rex@mail.com".to_string(),
            password: "rex_pwd".to_string(),
            status: 0,
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
}
