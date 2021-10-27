use super::access_token_generator::AccessTokenGenerator;
use crate::services::token_services::{
    access_token_claims::AccessTokenClaims, jwt_token_parameters::JwtTokenParameters,
};
use jsonwebtoken::{encode, Header};
use shaku::Component;
use std::{
    ops::Add,
    time::{self, Duration, SystemTime},
};

#[derive(Component)]
#[shaku(interface = AccessTokenGenerator)]
pub struct JwtAccessTokenGenerator {
    jwt_parameters: JwtTokenParameters,
}

impl AccessTokenGenerator for JwtAccessTokenGenerator {
    fn generate_token(&self, user: crate::models::user::User) -> String {
        let exp = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("Unable to get the current time")
            .add(Duration::from_secs(
                self.jwt_parameters.expiration as u64 * 60,
            ))
            .as_secs() as usize;

        let claims = AccessTokenClaims {
            iss: self.jwt_parameters.issuer.clone(),
            exp,
            email: user.email,
            id: user.id,
        };

        encode(
            &Header::default(),
            &claims,
            &self.jwt_parameters.encoding_key,
        )
        .expect("Error while encoding the JWT")
    }
}
