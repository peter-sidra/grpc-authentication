use crate::connection_pool_wrapper::DBConnectionPool;
use crate::services::{
    password_hashers::scrypt_hasher::ScryptHasher,
    refresh_token_repos::database_refresh_token_repo::DatabaseRefreshTokenRepo,
    token_services::{
        jwt_token_parameters::JwtTokenParameters,
        token_authenticator::TokenAuthenticatorImpl,
        token_generators::{
            jwt_access_token_generator::JwtAccessTokenGenerator,
            jwt_refresh_token_generator::JwtRefreshTokenGenerator,
        },
        token_validators::{
            jwt_access_token_validator::JwtAccessTokenValidator,
            jwt_refresh_token_validator::JwtRefreshTokenValidator,
        },
    },
    user_repos::database_user_repo::DatabaseUserRepo,
};
use config::Config;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use shaku::module;
use state::Storage;

// Init DI container
module! {
    pub AuthModule{
        components = [DBConnectionPool, DatabaseUserRepo, DatabaseRefreshTokenRepo,
                      ScryptHasher, JwtAccessTokenGenerator, JwtRefreshTokenGenerator,
                      JwtAccessTokenValidator, JwtRefreshTokenValidator, TokenAuthenticatorImpl],
        providers = [],
    }
}
pub static AUTH_MODULE: Storage<AuthModule> = Storage::new();

pub fn init_services(config: &Config) {
    // Setup jwt parameters
    let jwt_access_token_parameters = JwtTokenParameters {
        issuer: config.jwt_settings.issuer.clone(),
        encoding_key: EncodingKey::from_base64_secret(
            config.jwt_settings.access_token_key.as_str(),
        )
        .expect("Failed to load the jwt access token secret"),
        decoding_key: DecodingKey::from_base64_secret(
            config.jwt_settings.access_token_key.as_str(),
        )
        .expect("Failed to load the jwt access token secret"),
        expiration: config.jwt_settings.access_token_expiration_minutes,
    };

    let jwt_refresh_token_parameters = JwtTokenParameters {
        issuer: config.jwt_settings.issuer.clone(),
        encoding_key: EncodingKey::from_base64_secret(
            config.jwt_settings.refresh_token_key.as_str(),
        )
        .expect("Failed to load the jwt refresh token secret"),
        decoding_key: DecodingKey::from_base64_secret(
            config.jwt_settings.refresh_token_key.as_str(),
        )
        .expect("Failed to load the jwt refresh token secret"),
        expiration: config.jwt_settings.refresh_token_expiration_minutes,
    };

    // Setup jwt verification parameters
    let jwt_verification_options = Validation {
        iss: Some(config.jwt_settings.issuer.clone()),
        ..Default::default()
    };

    // Wire up the DI container
    use crate::services::password_hashers::scrypt_hasher::ScryptHasherParameters;
    use crate::services::token_services::token_generators::{
        jwt_access_token_generator::JwtAccessTokenGeneratorParameters,
        jwt_refresh_token_generator::JwtRefreshTokenGeneratorParameters,
    };
    use crate::services::token_services::token_validators::jwt_access_token_validator::JwtAccessTokenValidatorParameters;
    use crate::services::token_services::token_validators::jwt_refresh_token_validator::JwtRefreshTokenValidatorParameters;
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
                    jwt_parameters: jwt_refresh_token_parameters.clone(),
                },
            )
            .with_component_parameters::<JwtAccessTokenValidator>(
                JwtAccessTokenValidatorParameters {
                    jwt_parameters: jwt_access_token_parameters,
                    verification_options: jwt_verification_options.clone(),
                },
            )
            .with_component_parameters::<JwtRefreshTokenValidator>(
                JwtRefreshTokenValidatorParameters {
                    jwt_parameters: jwt_refresh_token_parameters,
                    verification_options: jwt_verification_options,
                },
            )
            .build(),
    );
}
