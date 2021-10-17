use super::access_token_generator::AccessTokenGenerator;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = AccessTokenGenerator)]
pub struct JwtAccessTokenGenerator {
    issuer: String,
    key: HS256Key,
    expiration: u32,
}

impl AccessTokenGenerator for JwtAccessTokenGenerator {
    fn generate_token(&self, user: crate::models::user::User) -> String {
        let claims = Claims::with_custom_claims(
            TokenClaims {
                id: user.id,
                email: user.email,
            },
            Duration::from_mins(self.expiration as u64),
        )
        .with_issuer(&self.issuer);

        self.key
            .authenticate(claims)
            .expect("Couldn't create the access token")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    id: String,
    email: String,
}
