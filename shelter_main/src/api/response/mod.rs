use serde::{Deserialize, Serialize};

pub mod login;
pub mod dog;
pub mod error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}