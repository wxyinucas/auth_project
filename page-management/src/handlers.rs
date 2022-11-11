use axum::headers::Cookie;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::{Extension, Form};

use util_auth::Claims;

use crate::forms::LoginForm;
use crate::{PMError, Result};

type TeraString = Html<String>;
pub type Redirect = (StatusCode, HeaderMap);

pub fn redirect_with_cookies(url: &str, cookies: Option<Cookie>) -> Result<Redirect> {
    todo!()
}

async fn page_login() -> Result<TeraString> {
    todo!()
}

async fn login(
    Form(form): Form<LoginForm>,
    // Extension(state): Extension<State>,
) -> Result<Redirect> {
    todo!()
}

async fn page_dashboard() -> Result<TeraString> {
    todo!()
}

async fn logout() -> Result<Redirect> {
    todo!()
}
// Extension(state): Extension<State>,
