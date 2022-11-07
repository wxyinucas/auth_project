use tera::Tera;

use util_auth::Jwt;

pub struct InnerState {
    pub tera: Tera,
    pub jwt: Jwt,
}
