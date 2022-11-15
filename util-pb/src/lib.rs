use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

pub use pb::user;

use crate::user::{AccountStatus, User};

mod pb;

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
