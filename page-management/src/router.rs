use axum::routing::get;
use axum::Router;

use crate::handlers::{
    handler_login, handler_user_add, handler_user_delete, handler_user_edit, page_dashboard,
    page_login, page_user_add, page_user_edit, page_user_index,
};

pub fn pm_router() -> Router {
    let user_router = Router::new()
        .route("/", get(page_user_index))
        .route("/add", get(page_user_add).post(handler_user_add))
        .route("/edit/:id", get(page_user_edit).post(handler_user_edit))
        .route("/delete/:id", get(handler_user_delete));

    Router::new()
        .route("/login", get(page_login).post(handler_login))
        .route("/dashboard", get(page_dashboard))
        .nest("/dashboard/users/", user_router)
    // .fallback(get(page_dashboard)) // TODO modify later
}
