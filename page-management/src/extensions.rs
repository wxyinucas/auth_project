use std::sync::Arc;
use tera::Tera;

use util_auth::Jwt;

pub struct InnerState {
    pub jwt: Jwt,
    pub tera: Tera,
}


pub struct State{
    pub inner: Arc<InnerState>,

}
