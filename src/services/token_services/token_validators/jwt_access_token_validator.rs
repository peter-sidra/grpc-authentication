use super::access_token_validator::AccessTokenValidator;
use crate::services::token_services::{
    access_token_claims::AccessTokenClaims, jwt_token_parameters::JwtTokenParameters,
};
use jwt_simple::prelude::*;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = AccessTokenValidator)]
pub struct JwtAccessTokenValidator {
    jwt_parameters: JwtTokenParameters,
    verification_options: VerificationOptions,
}

impl AccessTokenValidator for JwtAccessTokenValidator {
    fn validate_token(&self, token: &str) -> anyhow::Result<AccessTokenClaims> {
        // let claims = self
        //     .jwt_parameters
        //     .key
        //     .verify_token::<NoCustomClaims>(token, Some(self.verification_options.clone()))?;

        let claims = self
            .jwt_parameters
            .key
            .verify_token::<AccessTokenClaims>(token, Some(self.verification_options.clone()))?;

        Ok(claims.custom)
    }
}
