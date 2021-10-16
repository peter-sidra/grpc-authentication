pub mod proto_gen {
    tonic::include_proto!("authentication"); // The string specified here must match the proto package name
}

use proto_gen::{authenticator_server::Authenticator, RegisterRequest, RegisterResponse};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Scrypt,
};
use shaku::HasComponent;
use tonic::{Code, Request, Response, Status};

use crate::{models::user::NewUser, services::user_repos::user_repo::UserRepo, AUTH_MODULE};

#[derive(Debug, Default)]
pub struct MyAuthenticator {}

#[tonic::async_trait]
impl Authenticator for MyAuthenticator {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Creating user with email: {}", request.get_ref().email);

        let user_repo: &dyn UserRepo = AUTH_MODULE.get().resolve_ref();

        let email = request.get_ref().email.clone();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(request.get_ref().password.as_bytes(), &salt)
            .expect("Error while hashing a password")
            .to_string();
        let result = user_repo
            .create(NewUser {
                email,
                password_hash,
            })
            .await;

        match result {
            Ok(_) => Ok(Response::new(RegisterResponse::default())),
            Err(e) => match e {
                diesel::result::Error::DatabaseError(kind, _) => match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        Err(Status::new(Code::Unknown, "User already exists"))
                    }
                    _ => Err(Status::new(Code::Unknown, "Error while creating the user")),
                },
                _ => Err(Status::new(Code::Unknown, "Error while creating the user")),
            },
        }
    }
}
