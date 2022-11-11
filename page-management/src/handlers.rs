use std::sync::Arc;
use axum::headers::{Cookie, Header, HeaderValue};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::{Extension, Form};

use util_auth::Claims;

use crate::extensions::InnerState;
use crate::forms::LoginForm;
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

pub async fn page_login(Extension(state): Extension<Arc<InnerState>>) -> Result<TeraString> {
    let ctx = tera::Context::new();
    let page = state
        .tera
        .render("login.html", &ctx)
        .map_err(PMError::from)?;

    Ok(Html(page))
}

pub async fn login(
    Form(form): Form<LoginForm>,
    // Extension(state): Extension<State>,
) -> Result<Redirect> {
    todo!()
}

pub async fn page_dashboard() -> Result<TeraString> {
    todo!()
}

pub async fn logout() -> Result<Redirect> {
    todo!()
}
// Extension(state): Extension<State>,
