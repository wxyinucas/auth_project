use chrono::{Duration, Utc};
use jsonwebtoken as jwt;

pub use error::{AuthError, Result};

mod error;

pub struct Jwt {
    pub iss: String,
    pub exp: i64,
    pub secret: String,
}

// TODO 这个数据结构将来可能有变化，围绕他的接口如何制定？
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq)]
pub struct Claims {
    pub iss: String,
    pub exp: usize,
    pub email: String,
}

impl Jwt {
    pub fn new(iss: String, exp: i64, secret: String) -> Self {
        Self { iss, exp, secret }
    }

    fn cal_claims_exp(&self) -> usize {
        (Utc::now() + Duration::seconds(self.exp)).timestamp_millis() as usize
    }

    pub fn new_claims(&self, email: &str) -> Claims {
        Claims {
            iss: self.iss.clone(),
            exp: self.cal_claims_exp(),
            email: email.to_owned(),
        }
    }

    pub fn token(&self, claims: &Claims) -> Result<String> {
        let header = jwt::Header::new(jwt::Algorithm::HS256);
        let key = jwt::EncodingKey::from_secret(self.secret.as_bytes());

        let token = jwt::encode(&header, &claims, &key).map_err(AuthError::from)?;

        Ok(token)
    }

    pub fn valid_then_get_claim(&self, token: &str) -> Result<Claims> {
        let key = jwt::DecodingKey::from_secret(self.secret.as_bytes());
        let validation = jwt::Validation::new(jwt::Algorithm::HS256);
        let claims = jwt::decode::<Claims>(token, &key, &validation).map_err(AuthError::from)?;
        Ok(claims.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jwt_should_work() {
        let jwt = Jwt::new("Rex Wang".to_string(), 300, "Rex Secret".to_string());
        let origin_claims = jwt.new_claims("rex@gmail.com");

        let token = jwt.token(&origin_claims).unwrap();
        let claims = jwt.valid_then_get_claim(&token).unwrap();
        assert_eq!(claims, origin_claims);
    }
}
