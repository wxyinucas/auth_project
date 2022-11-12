use std::ops::Deref;
use std::sync::Arc;

use tera::Tera;

use util_auth::Jwt;

// TODO 将来增加数据库的占位
pub struct InnerState {
    pub jwt: Jwt,
    pub tera: Tera,
}

pub struct State {
    pub inner: Arc<InnerState>,
}

impl State {
    pub fn new(inner_state: InnerState) -> Self {
        Self {
            inner: Arc::new(inner_state),
        }
    }
}

impl Deref for State {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
