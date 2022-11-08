use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::Extension;
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
    Extension(inner_state): Extension<InnerState>,
) -> Result<Html<String>, PageAuthError> {
    let ctx = Context::new();
    let res = inner_state.tera.render("login.html", &ctx)?;
    Ok(Html(res))
}

pub async fn log(
    Extension(inner_state): Extension<InnerState>,
    form: crate::form::Login,
) -> Result<(StatusCode, HeaderMap), PageAuthError> {
    todo!()
}

pub async fn log_out() -> Result<(StatusCode, HeaderMap), PageAuthError> {
    todo!()
}
