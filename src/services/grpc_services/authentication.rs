pub mod proto_gen {
    tonic::include_proto!("authentication"); // The string specified here must match the proto package name
}

use proto_gen::{authenticator_server::Authenticator, RegisterRequest, RegisterResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyAuthenticator {}

#[tonic::async_trait]
impl Authenticator for MyAuthenticator {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Creating user with email: {}", request.get_ref().email);
        Ok(Response::new(RegisterResponse::default()))
    }
}
