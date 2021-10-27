use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub exp: usize,
    pub iss: String,
}
