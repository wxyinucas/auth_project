use error::Result;
use util_pb::user::{query_user_request, User};

pub mod db_pool;
pub mod error;

pub(crate) type UserId = i32;

#[tonic::async_trait]
trait UserDB {
    async fn query(&self, identity: query_user_request::Identity) -> Result<Option<User>>;

    async fn insert(&self, user: &User) -> Result<UserId>;

    async fn delete(&self, id: UserId) -> Result<User>;
}
