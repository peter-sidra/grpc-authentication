use super::token_generators::{
    access_token_generator::AccessTokenGenerator, refresh_token_generator::RefreshTokenGenerator,
};
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait TokenAuthenticator: Interface + AccessTokenGenerator + RefreshTokenGenerator {
    fn generate_access_token(&self, user: crate::models::user::User) -> String;
    fn generate_refresh_token(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = TokenAuthenticator)]
pub struct TokenAuthenticatorImpl {
    #[shaku(inject)]
    access_token_generator: Arc<dyn AccessTokenGenerator>,
    #[shaku(inject)]
    refresh_token_generator: Arc<dyn RefreshTokenGenerator>,
}

impl TokenAuthenticator for TokenAuthenticatorImpl {
    fn generate_access_token(&self, user: crate::models::user::User) -> String {
        AccessTokenGenerator::generate_token(self, user)
    }

    fn generate_refresh_token(&self) -> String {
        RefreshTokenGenerator::generate_token(self)
    }
}

impl AccessTokenGenerator for TokenAuthenticatorImpl {
    fn generate_token(&self, user: crate::models::user::User) -> String {
        self.access_token_generator.generate_token(user)
    }
}

impl RefreshTokenGenerator for TokenAuthenticatorImpl {
    fn generate_token(&self) -> String {
        self.refresh_token_generator.generate_token()
    }
}
