use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use error::AuthError;

pub mod error;
pub mod password;

// TODO 注意Jwt 将 Claims 中什么信息接管了，为什么这么设计？
// TODO Jwt 过期，有什么高级用法？
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub iss: String,
    pub exp: usize,
}

pub struct Jwt {
    pub secret: String,
    pub exp: i64,
    pub iss: String,
}

impl Jwt {
    pub fn new(secret: String, exp: i64, iss: String) -> Self {
        Self { secret, exp, iss }
    }

    fn cal_claims_exp(&self) -> usize {
        (Utc::now() + Duration::seconds(self.exp)).timestamp_millis() as usize
    }

    fn secret_bytes(&self) -> &[u8] {
        self.secret.as_bytes()
    }

    pub fn new_claims(&self, id: i32, email: String) -> Claims {
        Claims {
            id,
            email,
            iss: self.iss.to_string(),
            exp: self.cal_claims_exp(),
        }
    }

    pub fn new_claims_with(&self, claims: &Claims) -> Claims {
        self.new_claims(claims.id, claims.email.to_owned())
    }

    pub fn token(&self, claims: &Claims) -> Result<String, AuthError> {
        encode(
            // TODO: this function and all parameters.
            &Header::new(jsonwebtoken::Algorithm::HS256),
            claims,
            &EncodingKey::from_secret(self.secret_bytes()),
        )
        .map_err(AuthError::from)
    }

    pub fn verify_and_get_claims(&self, token: &str) -> Result<Claims, AuthError> {
        let mut valid = Validation::new(jsonwebtoken::Algorithm::HS256);
        valid.set_issuer(&[self.iss.clone()]);
        let token_data = decode(
            token,
            &DecodingKey::from_secret(self.secret_bytes()),
            &valid,
        )
        .map_err(AuthError::from)?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;

    const SECRET: &str = "secret";
    const ISS: &str = "rex wang";

    #[test]
    fn token_validation_should_work() {
        let jwt = Jwt::new(SECRET.to_string(), 300, ISS.to_string());
        let origin_claims = jwt.new_claims(42, "rex@gmail.com".to_owned());
        let token = jwt.token(&origin_claims).unwrap();

        let claims = jwt.verify_and_get_claims(&token).unwrap();
        assert_eq!(claims, origin_claims);
    }
}
