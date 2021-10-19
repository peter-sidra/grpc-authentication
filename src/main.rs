#[macro_use]
extern crate diesel;

mod config_loader;
mod connection_pool_wrapper;
mod di;
mod models;
mod schema;
mod services;

use config_loader::Config;
use config_loader::ConfigLoader;
use services::grpc_services::authentication::{
    proto_gen::authenticator_server::AuthenticatorServer, MyAuthenticator,
};
use services::grpc_services::helloworld::{proto_gen::greeter_server::GreeterServer, MyGreeter};
use state::LocalStorage;
use tonic::transport::Server;
use tonic::transport::ServerTlsConfig;

use crate::di::init_services;
use crate::services::interceptors::access_token_interceptor::check_auth;

// Global config
static CONFIG: LocalStorage<Config> = LocalStorage::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server configuration from config file
    CONFIG.set(|| ConfigLoader::load_config());
    let config = CONFIG.get();
    println!("Server config: \n{:?}", config);

    // Wire up the DI container
    init_services(&config);

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

    let greeter_server = GreeterServer::with_interceptor(MyGreeter::default(), check_auth);

    server
        .add_service(greeter_server)
        .add_service(AuthenticatorServer::new(MyAuthenticator::default()))
        .serve(addr)
        .await?;

    Ok(())
}
