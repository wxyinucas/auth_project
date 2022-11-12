use axum::headers::{Cookie, Header, HeaderValue};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::{Extension, Form};

use util_auth::Claims;

use crate::extensions::State;
use crate::forms::LoginForm;
use crate::middleware::CommonClaims;
use crate::{PMError, Result};

type TeraString = Html<String>;
pub type Redirect = (StatusCode, HeaderMap);

pub fn redirect_with_cookies(url: &str, cookies: Option<Cookie>) -> Result<Redirect> {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::LOCATION, url.parse()?);

    if let Some(cookies) = cookies {
        let mut vec: Vec<HeaderValue> = Vec::new();
        cookies.encode(&mut vec);
        for v in vec.iter() {
            headers.insert(axum::http::header::COOKIE, v.to_owned());
        }
    }

    Ok((StatusCode::FOUND, headers))
}

pub async fn page_login(Extension(state): Extension<State>) -> Result<TeraString> {
    let ctx = tera::Context::new();
    let page = state
        .tera
        .render("login.html", &ctx)
        .map_err(PMError::from)?;

    Ok(Html(page))
}

pub async fn login(
    Form(form): Form<LoginForm>,
    Extension(state): Extension<State>,
) -> Result<Redirect> {
    dotenv::dotenv().ok();
    use std::env::var;

    println!("activate!");
    // TODO 增加查询功能
    if form.email != "rex@mail.com" || form.password != "rex" {
        return Err(PMError::AuthError);
    }

    let claims = state.jwt.new_claims(&form.email);
    let token = state.jwt.token(&claims)?;
    let value =
        HeaderValue::from_str(&format!("{}={}", var("TOKEN_COOKIE").unwrap(), token)).unwrap();
    let cookies = Cookie::decode(&mut [&value].into_iter())?;

    redirect_with_cookies(&var("URL_DASHBOARD").unwrap(), Some(cookies))
}

pub async fn page_dashboard(
    CommonClaims(claims): CommonClaims<Claims>,
    Extension(state): Extension<State>,
) -> Result<TeraString> {
    let mut ctx = tera::Context::new();
    ctx.insert("email", &claims.email);
    let page = state
        .tera
        .render("dashboard.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

pub async fn logout() -> Result<Redirect> {
    todo!()
}
// Extension(state): Extension<State>,
