use tonic::{Request, Response, Status};

use util_pb::user::user_service_server::UserService;
use util_pb::user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, QueryUserRequest,
    QueryUserResponse,
};

use crate::{db_pool::UserDBPool, UserDB};

pub struct UserInnerService {
    pub db_pool: UserDBPool,
}
impl UserInnerService {
    pub fn new(db_pool: UserDBPool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl UserService for UserInnerService {
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

#[cfg(test)]
mod tests {
    use sqlx_db_tester::TestDb;

    use util_pb::user::query_user_request::Identity;

    use super::*;

    #[tokio::test]
    async fn user_inner_service_should_work() {
        println!("{:?}", std::env::current_dir());
        let tdb = TestDb::new("localhost", 5432, "", "", "../migrations"); // TODO test 从环境中读信息
        let pool = tdb.get_pool().await;
        let user_db_pool = UserDBPool::new(pool);

        let inner_service = UserInnerService::new(user_db_pool);

        // insert
        let create_req = CreateUserRequest {
            email: "rex@mail.com".to_string(),
            password: "rex".to_string(),
        };
        let create_req = tonic::Request::new(create_req);
        let create_res = inner_service.create(create_req).await.unwrap();
        let id = create_res.into_inner().id;
        assert_eq!(id, 1);

        // query
        let query_req1 = Request::new(QueryUserRequest {
            identity: Some(Identity::Id(id)),
        });
        let query_res1 = inner_service.query(query_req1).await.unwrap().into_inner();

        let query_req2 = Request::new(QueryUserRequest {
            identity: Some(Identity::Email("rex@mail.com".to_string())),
        });
        let query_res2 = inner_service.query(query_req2).await.unwrap().into_inner();
        assert_eq!(query_res1, query_res2);

        // delete
        let delete_req = Request::new(DeleteUserRequest {
            identity: Some(util_pb::user::delete_user_request::Identity::Id(1)),
        });
        let delete_res = inner_service.delete(delete_req).await.unwrap().into_inner();
        assert_eq!(delete_res.user.unwrap().password, String::from("rex"));
    }
}
