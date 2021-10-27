use super::refresh_token_validator::RefreshTokenValidator;
use crate::services::token_services::{
    jwt_token_parameters::JwtTokenParameters, refresh_token_claims::RefreshTokenClaims,
};
use jsonwebtoken::{decode, Validation};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = RefreshTokenValidator)]
pub struct JwtRefreshTokenValidator {
    jwt_parameters: JwtTokenParameters,
    verification_options: Validation,
}

impl RefreshTokenValidator for JwtRefreshTokenValidator {
    fn validate_token(&self, token: &str) -> anyhow::Result<()> {
        decode::<RefreshTokenClaims>(
            token,
            &self.jwt_parameters.decoding_key,
            &self.verification_options,
        )?;

        Ok(())
    }
}
