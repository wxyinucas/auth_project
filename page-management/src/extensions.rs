use tera::Tera;

use util_auth::Jwt;

pub struct InnerState {
    pub jwt: Jwt,
    pub tera: Tera,
}
