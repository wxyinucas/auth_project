use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub trait Claims: Serialize +  DeserializeOwned +Default {
    fn iss(self, iss: &str) -> Self;
    fn exp(self, exp: usize) -> Self;
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TestClaims {
    pub iss: String,
    pub exp: usize,
    pub name: String,
}

impl Claims for TestClaims {
    fn iss(self, iss: &str) -> Self {
        Self {
            iss: iss.to_string(),
            ..self
        }
    }

    fn exp(self, exp: usize) -> Self {
        Self { exp, ..self }
    }
}
