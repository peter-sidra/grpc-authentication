use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub id: String,
    pub email: String,
    pub exp: usize,
    pub iss: String,
}
