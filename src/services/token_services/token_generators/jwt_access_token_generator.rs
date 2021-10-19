use super::access_token_generator::AccessTokenGenerator;
use crate::services::token_services::{
    access_token_claims::AccessTokenClaims, jwt_token_parameters::JwtTokenParameters,
};
use jwt_simple::prelude::*;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = AccessTokenGenerator)]
pub struct JwtAccessTokenGenerator {
    jwt_parameters: JwtTokenParameters,
}

impl AccessTokenGenerator for JwtAccessTokenGenerator {
    fn generate_token(&self, user: crate::models::user::User) -> String {
        let claims = Claims::with_custom_claims(
            AccessTokenClaims {
                id: user.id,
                email: user.email,
            },
            Duration::from_mins(self.jwt_parameters.expiration as u64),
        )
        .with_issuer(&self.jwt_parameters.issuer);

        self.jwt_parameters
            .key
            .authenticate(claims)
            .expect("Couldn't create the access token")
    }
}
