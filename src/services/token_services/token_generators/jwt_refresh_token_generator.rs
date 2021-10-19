use super::refresh_token_generator::RefreshTokenGenerator;
use crate::services::token_services::jwt_token_parameters::JwtTokenParameters;
use jwt_simple::prelude::{Claims, Duration, MACLike};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = RefreshTokenGenerator)]
pub struct JwtRefreshTokenGenerator {
    jwt_parameters: JwtTokenParameters,
}

impl RefreshTokenGenerator for JwtRefreshTokenGenerator {
    fn generate_token(&self) -> String {
        let claims = Claims::create(Duration::from_mins(self.jwt_parameters.expiration as u64))
            .with_issuer(&self.jwt_parameters.issuer);

        self.jwt_parameters
            .key
            .authenticate(claims)
            .expect("Couldn't create the refresh token")
    }
}
