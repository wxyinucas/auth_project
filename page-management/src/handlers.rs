use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{Extension, Form};
use axum_macros::debug_handler;
use serde::Deserialize;
use tera::Context;

use crate::{error::Result, PMError, State, UserClaims, DASHBOARD, TOKEN_COOKIE};

type TeraHtml = Html<String>;
pub type Redirect = (StatusCode, HeaderMap);

pub fn redirect_with_cookies(url: &str, cookies: Option<&str>) -> Redirect {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        url.parse().expect("URL parse failed"),
    );

    if let Some(cookies) = cookies {
        headers.insert(axum::http::header::SET_COOKIE, cookies.parse().unwrap());
    }
    (StatusCode::FOUND, headers)
}

/* =================================================================


    handlers


================================================================== */

pub async fn page_login(Extension(state): Extension<State>) -> Result<TeraHtml> {
    let ctx = Context::new();
    let page = state
        .tera
        .render("sign_in.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

#[debug_handler]
pub async fn handler_login(
    Form(form): Form<LoginForm>,
    Extension(state): Extension<State>,
) -> Result<Redirect> {
    // TODO implement Auth check
    let raw_claims = UserClaims {
        email: form.email,
        ..UserClaims::default()
    };
    let claims = state.jwt.new_claims(raw_claims).map_err(PMError::from)?;
    let token = state.jwt.token(claims).map_err(PMError::from)?;
    let inner = &format!("{}={}", TOKEN_COOKIE, token)[..];
    let cookie = Some(inner);
    Ok(redirect_with_cookies(DASHBOARD, cookie))
}

pub async fn page_dashboard(Extension(state): Extension<State>) -> Result<TeraHtml> {
    let ctx = Context::new();
    let page = state
        .tera
        .render("dashboard.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

/* =================================================================


    Forms


================================================================== */

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
