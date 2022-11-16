use tonic::{Request, Response, Status};

use util_pb::user::user_service_server::UserService;
use util_pb::user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, QueryUserRequest,
    QueryUserResponse,
};

use crate::{db_pool::UserDBPool, UserDB};

pub struct InnerUserService {
    pub db_pool: UserDBPool,
}

#[tonic::async_trait]
impl UserService for InnerUserService {
    async fn create(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let user_id = self
            .db_pool
            .insert(&req)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(CreateUserResponse { id: user_id }))
    }

    async fn query(
        &self,
        request: Request<QueryUserRequest>,
    ) -> Result<Response<QueryUserResponse>, Status> {
        let req = request.into_inner();
        let users = self
            .db_pool
            .query(req.identity.unwrap())
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(QueryUserResponse { users }))
    }

    async fn delete(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .db_pool
            .query(req.identity.unwrap().into())
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        if user.is_none() {
            return Err(Status::data_loss("User does not exist"));
        }
        let user = self
            .db_pool
            .delete(user.unwrap().id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(DeleteUserResponse { user: Some(user) }))
    }
}
