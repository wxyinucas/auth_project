#[derive(sqlx::FromRow)]
#[derive(serde::Serialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration = "UserAuthLevel", tag = "5")]
    pub auth: i32,
    #[prost(enumeration = "UserStatus", tag = "6")]
    pub status: i32,
}
#[derive(sqlx::Type)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserAuthLevel {
    Admin = 0,
    Customer = 1,
}
impl UserAuthLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserAuthLevel::Admin => "ADMIN",
            UserAuthLevel::Customer => "CUSTOMER",
        }
    }
}
#[derive(sqlx::Type)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserStatus {
    Active = 0,
    Freeze = 1,
    Dropped = 2,
}
impl UserStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserStatus::Active => "ACTIVE",
            UserStatus::Freeze => "FREEZE",
            UserStatus::Dropped => "DROPPED",
        }
    }
}
