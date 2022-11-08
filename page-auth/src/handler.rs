use std::sync::Arc;

use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::{Extension, Form};
use sqlx::PgPool;
use tera::Context;

use crate::error::PageAuthError;
use crate::structs::InnerState;

fn redirect(url: &str) -> (StatusCode, HeaderMap) {
    redirect_with_cookie(url, None)
}

fn redirect_with_cookie(url: &str, cookie: Option<&str>) -> (StatusCode, HeaderMap) {
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.parse().unwrap());

    if let Some(cookie) = cookie {
        header.insert(axum::http::header::COOKIE, cookie.parse().unwrap());
    }

    (StatusCode::FOUND, header)
}

pub async fn index() -> (StatusCode, HeaderMap) {
    redirect("/login")
}

pub async fn login_page(
    Extension(inner_state): Extension<Arc<InnerState>>,
) -> Result<Html<String>, PageAuthError> {
    let ctx = Context::new();
    let res = inner_state.tera.render("login.html", &ctx)?;
    Ok(Html(res))
}

pub async fn log(
    Extension(inner_state): Extension<Arc<InnerState>>,
    Form(form): Form<crate::form::Login>,
) -> Result<(StatusCode, HeaderMap), PageAuthError> {
    let pool = PgPool::connect("postgres://localhost:5432/new_db")
        .await
        .unwrap();
    let res = sqlx::query_as::<_, util_pb::User>(
        "SELECT * FROM auth.users WHERE email=$1 AND password=$2",
    )
    .bind(&form.email)
    .bind(&form.password)
    .fetch_optional(&pool)
    .await
    .map_err(PageAuthError::from)?;

    println!("{:?}", res);

    let user = match res {
        None => return Err(PageAuthError::LoginFailed),
        Some(user) => user,
    };

    let claims = inner_state.jwt.new_claims(user.id, user.email);
    let token = inner_state
        .jwt
        .token(&claims)
        .map_err(PageAuthError::from)?;
    let cookie = format!("rex_token={}", token);
    Ok(redirect_with_cookie("/login_success", Some(&cookie)))
}

pub async fn log_out() -> Result<(StatusCode, HeaderMap), PageAuthError> {
    Ok(redirect_with_cookie("/login", Some("rex_token=")))
}

pub async fn login_success_page(
    Extension(inner_state): Extension<Arc<InnerState>>,
) -> Result<Html<String>, PageAuthError> {
    let ctx = Context::new();
    let res = inner_state.tera.render("add.html", &ctx)?;
    Ok(Html(res))
}
