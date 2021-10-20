use super::refresh_token_validator::RefreshTokenValidator;
use crate::services::token_services::jwt_token_parameters::JwtTokenParameters;
use jwt_simple::prelude::*;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = RefreshTokenValidator)]
pub struct JwtRefreshTokenValidator {
    jwt_parameters: JwtTokenParameters,
    verification_options: VerificationOptions,
}

impl RefreshTokenValidator for JwtRefreshTokenValidator {
    fn validate_token(&self, token: &str) -> anyhow::Result<()> {
        let _ = self
            .jwt_parameters
            .key
            .verify_token::<NoCustomClaims>(token, Some(self.verification_options.clone()))?;

        Ok(())
    }
}
