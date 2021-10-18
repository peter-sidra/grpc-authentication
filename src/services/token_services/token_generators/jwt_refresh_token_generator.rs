use jwt_simple::prelude::{Claims, Duration, HS256Key, MACLike};
use shaku::Component;

use super::refresh_token_generator::RefreshTokenGenerator;

#[derive(Component)]
#[shaku(interface = RefreshTokenGenerator)]
pub struct JwtRefreshTokenGenerator {
    issuer: String,
    key: HS256Key,
    expiration: u32,
}

impl RefreshTokenGenerator for JwtRefreshTokenGenerator {
    fn generate_token(&self) -> String {
        let claims =
            Claims::create(Duration::from_mins(self.expiration as u64)).with_issuer(&self.issuer);

        self.key
            .authenticate(claims)
            .expect("Couldn't create the refresh token")
    }
}
