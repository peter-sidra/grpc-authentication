use crate::connection_pool_wrapper::DBConnectionPool;
use crate::{
    config_loader::Config,
    services::{
        password_hashers::scrypt_hasher::ScryptHasher,
        token_services::{
            jwt_token_parameters::JwtTokenParameters,
            token_authenticator::TokenAuthenticatorImpl,
            token_generators::{
                jwt_access_token_generator::JwtAccessTokenGenerator,
                jwt_refresh_token_generator::JwtRefreshTokenGenerator,
            },
            token_validators::jwt_access_token_validator::JwtAccessTokenValidator,
        },
        user_repos::database_user_repo::DatabaseUserRepo,
    },
};
use jwt_simple::prelude::*;
use shaku::module;
use state::Storage;

// Init DI container
module! {
    pub AuthModule{
        components = [DBConnectionPool, DatabaseUserRepo, ScryptHasher,
                      JwtAccessTokenGenerator, JwtRefreshTokenGenerator,
                      JwtAccessTokenValidator, TokenAuthenticatorImpl],
        providers = [],
    }
}
pub static AUTH_MODULE: Storage<AuthModule> = Storage::new();

pub fn init_services(config: &Config) {
    // Setup jwt parameters
    let jwt_access_token_parameters = JwtTokenParameters {
        issuer: config.jwt_settings.issuer.clone(),
        key: jwt_simple::prelude::HS256Key::from_bytes(
            config.jwt_settings.access_token_key.as_bytes(),
        ),
        expiration: config.jwt_settings.access_token_expiration_minutes,
    };

    let jwt_refresh_token_parameters = JwtTokenParameters {
        issuer: config.jwt_settings.issuer.clone(),
        key: jwt_simple::prelude::HS256Key::from_bytes(
            config.jwt_settings.refresh_token_key.as_bytes(),
        ),
        expiration: config.jwt_settings.refresh_token_expiration_minutes,
    };

    // Setup jwt verification parameters
    let mut jwt_access_token_verification_options = VerificationOptions::default();
    jwt_access_token_verification_options.allowed_issuers =
        Some(std::collections::HashSet::from_strings(&[
            jwt_access_token_parameters.issuer.as_str(),
        ]));

    // Wire up the DI container
    use crate::services::password_hashers::scrypt_hasher::ScryptHasherParameters;
    use crate::services::token_services::token_generators::{
        jwt_access_token_generator::JwtAccessTokenGeneratorParameters,
        jwt_refresh_token_generator::JwtRefreshTokenGeneratorParameters,
    };
    use crate::services::token_services::token_validators::jwt_access_token_validator::JwtAccessTokenValidatorParameters;
    AUTH_MODULE.set(
        AuthModule::builder()
            // Setup the access token generator
            .with_component_parameters::<DBConnectionPool>(config.database_url.clone())
            .with_component_parameters::<ScryptHasher>(ScryptHasherParameters {
                work_factor: config.password_work_factor,
            })
            .with_component_parameters::<JwtAccessTokenGenerator>(
                JwtAccessTokenGeneratorParameters {
                    jwt_parameters: jwt_access_token_parameters.clone(),
                },
            )
            .with_component_parameters::<JwtRefreshTokenGenerator>(
                JwtRefreshTokenGeneratorParameters {
                    jwt_parameters: jwt_refresh_token_parameters,
                },
            )
            .with_component_parameters::<JwtAccessTokenValidator>(
                JwtAccessTokenValidatorParameters {
                    jwt_parameters: jwt_access_token_parameters,
                    verification_options: jwt_access_token_verification_options,
                },
            )
            .build(),
    );
}
