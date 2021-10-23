// use hello_world::greeter_server::{Greeter, GreeterServer};
// use hello_world::{HelloReply, HelloRequest};

pub mod proto_gen {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

use proto_gen::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

use crate::services::token_services::access_token_claims::AccessTokenClaims;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let claims = match request.extensions().get::<AccessTokenClaims>() {
            Some(value) => value,
            None => {
                eprintln!("Error while accessing claims in say_hello");
                return Err(Status::new(tonic::Code::Internal, "Internal error"));
            }
        };

        let reply = HelloReply {
            message: format!(
                "Hello {}, your email is {}",
                request.get_ref().name,
                claims.email
            ),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
