// use hello_world::greeter_server::{Greeter, GreeterServer};
// use hello_world::{HelloReply, HelloRequest};

pub mod proto_gen {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

use proto_gen::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        // println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}", request.get_ref().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        // Ok::<Response<HelloReply>, Status>(Response::new(reply.clone()))

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
