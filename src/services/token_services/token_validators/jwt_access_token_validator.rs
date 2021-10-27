use super::access_token_validator::AccessTokenValidator;
use crate::services::token_services::{
    access_token_claims::AccessTokenClaims, jwt_token_parameters::JwtTokenParameters,
};
use jsonwebtoken::{decode, Validation};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = AccessTokenValidator)]
pub struct JwtAccessTokenValidator {
    jwt_parameters: JwtTokenParameters,
    verification_options: Validation,
}

impl AccessTokenValidator for JwtAccessTokenValidator {
    fn validate_token(&self, token: &str) -> anyhow::Result<AccessTokenClaims> {
        let token_data = decode::<AccessTokenClaims>(
            token,
            &self.jwt_parameters.decoding_key,
            &self.verification_options,
        )?;

        Ok(token_data.claims)
    }
}
