use util_pb::user::{CreateUserRequest, QueryUserRequest, User};

use crate::error::Result;
use crate::UserId;

#[tonic::async_trait]
pub trait UserDB {
    async fn query(&self, req: QueryUserRequest) -> Result<Vec<User>>;

    async fn insert(&self, user: &CreateUserRequest) -> Result<UserId>;

    async fn delete(&self, id: UserId) -> Result<User>;
}
