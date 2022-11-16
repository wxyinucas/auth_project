use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

pub use pb::user;

use crate::user::delete_user_request::Identity;
use crate::user::{AccountStatus, User};

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


    From Identity to Identity


================================================================== */
impl From<Identity> for user::query_user_request::Identity {
    fn from(value: Identity) -> Self {
        match value {
            Identity::Id(id) => user::query_user_request::Identity::Id(id),
            Identity::Email(email) => user::query_user_request::Identity::Email(email),
        }
    }
}
