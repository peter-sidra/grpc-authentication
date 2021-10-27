use super::refresh_token_generator::RefreshTokenGenerator;
use crate::services::token_services::{
    jwt_token_parameters::JwtTokenParameters, refresh_token_claims::RefreshTokenClaims,
};
use jsonwebtoken::{encode, Header};
use shaku::Component;
use std::{
    ops::Add,
    time::{self, Duration, SystemTime},
};

#[derive(Component)]
#[shaku(interface = RefreshTokenGenerator)]
pub struct JwtRefreshTokenGenerator {
    jwt_parameters: JwtTokenParameters,
}

impl RefreshTokenGenerator for JwtRefreshTokenGenerator {
    fn generate_token(&self) -> String {
        let exp = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("Unable to get the current time")
            .add(Duration::from_secs(
                self.jwt_parameters.expiration as u64 * 60,
            ))
            .as_secs() as usize;

        let claims = RefreshTokenClaims {
            iss: self.jwt_parameters.issuer.clone(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &self.jwt_parameters.encoding_key,
        )
        .expect("Error while encoding the JWT")
    }
}
