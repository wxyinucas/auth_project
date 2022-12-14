use tonic::{Request, Response, Status};
use tracing::info;

use util_pb::user::user_service_server::UserService;
use util_pb::user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, QueryUserRequest,
    QueryUserResponse,
};

use crate::{db_pool::UserDBPool, traits::UserDB};

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

        info!("New User with id {} created", user_id);
        Ok(Response::new(CreateUserResponse { id: user_id }))
    }

    async fn query(
        &self,
        request: Request<QueryUserRequest>,
    ) -> Result<Response<QueryUserResponse>, Status> {
        let req = request.into_inner();
        let users = self
            .db_pool
            .query(req)
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
            .query(req.into())
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        if user.is_empty() {
            return Err(Status::data_loss("User does not exist"));
        } else if user.len() > 1 {
            return Err(Status::out_of_range("More than one user found"));
        }

        let user = self
            .db_pool
            .delete(user[0].id)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        info!("{:?} deleted", user);
        Ok(Response::new(DeleteUserResponse { user: Some(user) }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use dotenv::dotenv;
    use sqlx_db_tester::TestPg;

    use super::*;

    #[tokio::test]
    async fn user_inner_service_should_work() {
        dotenv().ok();
        println!("{:?}", std::env::current_dir());
        let tdb = TestPg::new(
            std::env::var("TDB_URL").unwrap(),
            Path::new("../migrations"),
        ); // TODO test ?????????????????????
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
            id,
            ..QueryUserRequest::default()
        });
        let query_res1 = inner_service.query(query_req1).await.unwrap().into_inner();

        let query_req2 = Request::new(QueryUserRequest {
            email: "rex@mail.com".to_string(),
            ..QueryUserRequest::default()
        });
        let query_res2 = inner_service.query(query_req2).await.unwrap().into_inner();
        assert_eq!(query_res1, query_res2);

        // delete
        let delete_req = Request::new(DeleteUserRequest { id: 1 });
        let delete_res = inner_service.delete(delete_req).await.unwrap().into_inner();
        assert_eq!(delete_res.user.unwrap().password, String::from("rex"));
    }
}
