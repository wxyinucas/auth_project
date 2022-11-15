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
                let sql = "SELECT * FROM users WHERE id = $1";
                sqlx::query_as(sql).bind(id)
            }
            query_user_request::Identity::Email(email) => {
                let sql = "SELECT * FROM users WHERE email = $1";
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
            .map_err(UsersError::from)?;
        Ok(res.get(0))
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
