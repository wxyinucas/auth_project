use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

use crate::{User, UserAuthLevel, UserStatus};

pub mod user;

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        // TODO 改进
        let id = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let password: String = row.get("password");
        // let auth_level = match row.get("auth_level") {
        //     UserAuthLevel::Admin => 0,
        //     UserAuthLevel::Customer => 1,
        // };
        // let status = match row.get("status") {
        //     UserStatus::Active => 0,
        //     UserStatus::Freeze => 1,
        //     UserStatus::Dropped => 2,
        // };
        let auth_level: AuthLevel = row.get("auth_level"); // TODO 抄的好呀！reservation 和 全局搜索。
        let status: Status = row.get("status"); // TODO 自己怎么看doc 能学会？试试写一个最短路径。

        let res = Self {
            id,
            name,
            email,
            password,
            auth_level: UserAuthLevel::from(auth_level) as i32,
            status: UserStatus::from(status) as i32,
        };

        Ok(res)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "user_auth_level", rename_all = "lowercase")]
enum AuthLevel {
    Admin,
    Customer,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
enum Status {
    Active,
    Freeze,
    Dropped,
}

impl From<AuthLevel> for UserAuthLevel {
    fn from(value: AuthLevel) -> Self {
        match value {
            AuthLevel::Admin => UserAuthLevel::Admin,
            AuthLevel::Customer => UserAuthLevel::Customer,
        }
    }
}

impl From<Status> for UserStatus {
    fn from(value: Status) -> Self {
        match value {
            Status::Active => UserStatus::Active,
            Status::Freeze => UserStatus::Freeze,
            Status::Dropped => UserStatus::Dropped,
        }
    }
}
