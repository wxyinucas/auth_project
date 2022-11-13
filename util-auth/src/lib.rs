use jsonwebtoken as jwt;
use jsonwebtoken::{DecodingKey, EncodingKey};

use claims::Claims;
pub use error::AuthError;
use error::Result;

pub mod claims;
mod error;

pub struct Jwt {
    pub iss: String,
    pub exp: usize,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl Jwt {
    pub fn new(iss: String, exp: usize, secret: &str) -> Self {
        Self {
            iss,
            exp,
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    fn get_epoch(exp: usize) -> usize {
        use std::time::SystemTime;
        let res = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        res + exp
    }

    pub fn new_claims<T: Claims>(&self, claims: T) -> Result<T> {
        let claims = claims.iss(&self.iss);
        let claims = claims.exp(Self::get_epoch(self.exp));
        Ok(claims)
    }

    pub fn token<T: Claims>(&self, claims: T) -> Result<String> {
        let header = jwt::Header::new(jwt::Algorithm::HS256);
        let token = jwt::encode(&header, &claims, &self.encoding_key)?;

        Ok(token)
    }

    pub fn validate_and_get_claims<T: Claims>(&self, token: &str) -> Result<T> {
        let validation = jwt::Validation::new(jwt::Algorithm::HS256);
        let claims = jwt::decode::<T>(token, &self.decoding_key, &validation)?;

        Ok(claims.claims)
    }
}

#[cfg(test)]
mod tests {
    use crate::claims::TestClaims;

    use super::*;

    #[test]
    fn jwt_should_work() {
        let jwt = Jwt::new("rex wang".to_string(), 60, "secret");
        let test_claims = TestClaims {
            name: "test_claims".to_string(),
            ..TestClaims::default()
        };

        let origin_claims = jwt.new_claims(test_claims).unwrap();
        let token = jwt.token(origin_claims.clone()).unwrap();
        let claims = jwt
            .validate_and_get_claims::<TestClaims>(token.as_str())
            .unwrap();
        assert_eq!(origin_claims, claims);
    }
}
