use shaku::Interface;

use crate::services::token_services::access_token_claims::AccessTokenClaims;

pub trait AccessTokenValidator: Interface {
    fn validate_token(&self, token: &str) -> anyhow::Result<AccessTokenClaims>;
}
