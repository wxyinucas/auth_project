use util_pb::user::{query_user_request, CreateUserRequest, User};

use crate::error::Result;

pub(crate) type UserId = i32;

#[tonic::async_trait]
pub trait UserDB {
    async fn query(&self, identity: query_user_request::Identity) -> Result<Option<User>>;

    async fn insert(&self, user: &CreateUserRequest) -> Result<UserId>;

    async fn delete(&self, id: UserId) -> Result<User>;
}
