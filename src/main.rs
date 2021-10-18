#[macro_use]
extern crate diesel;

mod config_loader;
mod models;
mod schema;
mod services;

use config_loader::Config;
use config_loader::ConfigLoader;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;
use services::grpc_services::authentication::{
    proto_gen::authenticator_server::AuthenticatorServer, MyAuthenticator,
};
use services::grpc_services::helloworld::{proto_gen::greeter_server::GreeterServer, MyGreeter};
use services::password_hashers::scrypt_hasher::ScryptHasher;
use services::token_services::token_authenticator::TokenAuthenticatorImpl;
use services::token_services::token_generators::{
    jwt_access_token_generator::JwtAccessTokenGenerator,
    jwt_refresh_token_generator::JwtRefreshTokenGenerator,
};
use services::user_repos::database_user_repo::DatabaseUserRepo;
use shaku::module;
use state::LocalStorage;
use state::Storage;
use tonic::transport::Server;
use tonic::transport::ServerTlsConfig;

// Global config
static CONFIG: LocalStorage<Config> = LocalStorage::new();

// Init DI container
module! {
    AuthModule{
        components = [DatabaseUserRepo, ScryptHasher, JwtAccessTokenGenerator, JwtRefreshTokenGenerator, TokenAuthenticatorImpl],
        providers = [],
    }
}
static AUTH_MODULE: Storage<AuthModule> = Storage::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server configuration from config file
    CONFIG.set(|| ConfigLoader::load_config());
    let config = CONFIG.get();
    println!("Server config: \n{:?}", config);

    // Create the DB connection pool
    let pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(
            CONFIG.get().database_url.as_str(),
        ))
        .expect("Error while creating the database connection pool");

    // Wire up the DI container
    use crate::services::token_services::token_generators::{
        jwt_access_token_generator::JwtAccessTokenGeneratorParameters,
        jwt_refresh_token_generator::JwtRefreshTokenGeneratorParameters,
    };
    use crate::services::user_repos::database_user_repo::DatabaseUserRepoParameters;
    AUTH_MODULE.set(
        AuthModule::builder()
            // Setup the database user repo
            .with_component_parameters::<DatabaseUserRepo>(DatabaseUserRepoParameters {
                db_connection_pool: pool,
            })
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

    // Build the server
    let addr = config.server_addr.parse()?;

    let mut server = Server::builder();

    // Use tls if configured
    if config.use_tls {
        let cert = tokio::fs::read(config.cert_path.as_str())
            .await
            .expect("Couldn't read cert");
        let key = tokio::fs::read(config.key_path.as_str())
            .await
            .expect("Couldn't read key");

        let identity = tonic::transport::Identity::from_pem(cert, key);

        server = server
            .tls_config(ServerTlsConfig::new().identity(identity))
            .expect("Error while configuring TLS");
    }

    server
        .add_service(GreeterServer::new(MyGreeter::default()))
        .add_service(AuthenticatorServer::new(MyAuthenticator::default()))
        .serve(addr)
        .await?;

    Ok(())
}
