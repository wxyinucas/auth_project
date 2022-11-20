use axum::extract::Path;
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{Extension, Form};
use serde::Deserialize;
use tera::Context;

use svc_users::UserId;
use util_pb::user::{CreateUserRequest, DeleteUserRequest, QueryUserRequest};

use crate::{error::Result, CommonClaims, PMError, State, UserClaims, DASHBOARD, TOKEN_COOKIE};

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


    functional handlers


================================================================== */

pub async fn handler_login(
    Form(form): Form<LoginForm>,
    Extension(state): Extension<State>,
) -> Result<Redirect> {
    let query = QueryUserRequest {
        email: form.email.clone(),
        ..QueryUserRequest::default()
    };

    let user_client = state.user_client.clone();
    let user = user_client
        .ok_or(PMError::LackClientError("User"))
        .unwrap()
        .query(query)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            PMError::InnerSvcError("User".into())
        })?
        .into_inner()
        .users;
    if user.is_empty() || user[0].password != form.password {
        return Err(PMError::AuthError);
    }

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

pub async fn handler_user_add(
    Form(form): Form<AddUserForm>,
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<Redirect> {
    tracing::info!("User {} is creating.", &form.email);
    let req = CreateUserRequest {
        email: form.email,
        password: form.password,
    };

    let user_client = state.user_client.clone();
    let _res = user_client
        .unwrap()
        .create(req)
        .await
        .map_err(|e| PMError::InnerSvcError(e.to_string()))?;
    tracing::info!("Created");
    Ok(redirect_with_cookies(
        "/dashboard/users?msg='User added'",
        None,
    ))
}

pub async fn handler_user_edit(
    Form(_form): Form<EditForm>,
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(_state): Extension<State>,
) -> Result<Redirect> {
    todo!()
}

pub async fn handler_user_delete(
    Path(id): Path<UserId>,
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<Redirect> {
    tracing::info!("User {} is deleting.", id);
    let query = DeleteUserRequest { id };
    let user_client = state.user_client.clone();
    let _res = user_client
        .unwrap()
        .delete(query)
        .await
        .map_err(|err| PMError::InnerSvcError(err.to_string()))?;
    Ok(redirect_with_cookies(
        "/dashboard/users?msg='User deleted'",
        None,
    ))
}
/* =================================================================


    page handlers


================================================================== */
pub async fn page_login(Extension(state): Extension<State>) -> Result<TeraHtml> {
    let ctx = Context::new();
    let page = state
        .tera
        .render("sign_in.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

pub async fn page_dashboard(
    CommonClaims(claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<TeraHtml> {
    let ctx = Context::new();

    tracing::info!("{} logged in", claims.email);
    let page = state
        .tera
        .render("dashboard/base.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

pub async fn page_user_index(
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query = QueryUserRequest::default();
    let mut user_client = state.user_client.clone().unwrap();
    let res = user_client
        .query(query)
        .await
        .map_err(|err| PMError::InnerSvcError(err.to_string()))?;
    let list = res.into_inner().users;

    tracing::trace!("{:?}", list);
    ctx.insert("list", &list);

    let page = state
        .tera
        .render("dashboard/user_index.html", &ctx)
        .map_err(PMError::from)?;

    Ok(Html(page))
}

pub async fn page_user_edit(
    Path(id): Path<UserId>,
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();
    tracing::info!("User with id-{} is being edited.", id);

    let query = QueryUserRequest {
        id,
        ..QueryUserRequest::default()
    };
    let mut user_client = state.user_client.clone().unwrap();
    let res = user_client
        .query(query)
        .await
        .map_err(|err| PMError::InnerSvcError(err.to_string()))?;
    let user = res.into_inner().users.pop().unwrap();

    ctx.insert("user", &user);

    let page = state
        .tera
        .render("dashboard/user_edit.html", &ctx)
        .map_err(PMError::from)?;
    Ok(Html(page))
}

pub async fn page_user_add(
    CommonClaims(_claims): CommonClaims<UserClaims>,
    Extension(state): Extension<State>,
) -> Result<TeraHtml> {
    let ctx = Context::new();

    let page = state
        .tera
        .render("dashboard/user_add.html", &ctx)
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

#[derive(Debug, Deserialize)]
pub struct EditForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AddUserForm {
    pub email: String,
    pub password: String,
}
