use crate::connection_pool_wrapper::DBConnectionPool;
use crate::{
    config_loader::Config,
    services::{
        password_hashers::scrypt_hasher::ScryptHasher,
        token_services::{
            token_authenticator::TokenAuthenticatorImpl,
            token_generators::{
                jwt_access_token_generator::JwtAccessTokenGenerator,
                jwt_refresh_token_generator::JwtRefreshTokenGenerator,
            },
        },
        user_repos::database_user_repo::DatabaseUserRepo,
    },
};
use shaku::module;
use state::Storage;

// Init DI container
module! {
    pub AuthModule{
        components = [DBConnectionPool, DatabaseUserRepo, ScryptHasher,
                      JwtAccessTokenGenerator, JwtRefreshTokenGenerator,
                      TokenAuthenticatorImpl],
        providers = [],
    }
}
pub static AUTH_MODULE: Storage<AuthModule> = Storage::new();

pub fn di_wireup(config: &Config) {
    // Wire up the DI container
    use crate::services::token_services::token_generators::{
        jwt_access_token_generator::JwtAccessTokenGeneratorParameters,
        jwt_refresh_token_generator::JwtRefreshTokenGeneratorParameters,
    };
    AUTH_MODULE.set(
        AuthModule::builder()
            // Setup the access token generator
            .with_component_parameters::<JwtAccessTokenGenerator>(
                JwtAccessTokenGeneratorParameters {
                    issuer: config.jwt_settings.issuer.clone(),
                    key: jwt_simple::prelude::HS256Key::from_bytes(
                        config.jwt_settings.access_token_key.as_bytes(),
                    ),
                    expiration: config.jwt_settings.access_token_expiration_minutes,
                },
            )
            .with_component_parameters::<JwtRefreshTokenGenerator>(
                JwtRefreshTokenGeneratorParameters {
                    issuer: config.jwt_settings.issuer.clone(),
                    key: jwt_simple::prelude::HS256Key::from_bytes(
                        config.jwt_settings.refresh_token_key.as_bytes(),
                    ),
                    expiration: config.jwt_settings.refresh_token_expiration_minutes,
                },
            )
            .build(),
    );
}
