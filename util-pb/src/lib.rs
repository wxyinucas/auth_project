use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

pub use pb::user;

use crate::user::{AccountStatus, DeleteUserRequest, QueryUserRequest, User};

mod pb;

/* =================================================================


    FromRow for User


================================================================== */

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: i32 = row.get("id");
        let email: String = row.get("email");
        let password: String = row.get("password");
        let status: AS = row.get("status");

        Ok(User {
            id,
            email,
            password,
            status: AccountStatus::from(status) as i32,
        })
    }
}

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase", type_name = "user_status")]
pub enum AS {
    Active,
    Frozen,
}

impl From<AS> for AccountStatus {
    fn from(value: AS) -> Self {
        match value {
            AS::Active => AccountStatus::Active,
            AS::Frozen => AccountStatus::Frozen,
        }
    }
}
/* =================================================================


 Query to Sql


================================================================== */
pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for QueryUserRequest {
    fn to_sql(&self) -> String {
        let condition1 = if self.status == 0 {
            "True".into()
        } else {
            format!("status = {}", self.status)
        };

        let condition2 = if self.id == 0 {
            "True".into()
        } else {
            format!("id = {}", self.id)
        };
        let condition3 = if self.email.is_empty() {
            "True".into()
        } else {
            format!("email = '{}'", self.email)
        };

        format!(
            "SELECT * FROM auth.users WHERE {} AND {} AND {}",
            condition1, condition2, condition3
        )
    }
}
/* =================================================================


From DeleteReq to QueryReq


================================================================== */
impl From<DeleteUserRequest> for QueryUserRequest {
    fn from(value: DeleteUserRequest) -> Self {
        Self {
            id: value.id,
            email: "".to_string(),
            status: 0,
        }
    }
}
