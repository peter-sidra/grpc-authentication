pub mod proto_gen {
    tonic::include_proto!("authentication"); // The string specified here must match the proto package name
}

use crate::{
    models::user::NewUser,
    services::password_hashers::hasher::PasswordHasher as MyPasswordHasher,
    services::{
        token_services::token_authenticator::TokenAuthenticator, user_repos::user_repo::UserRepo,
    },
    AUTH_MODULE,
};
use proto_gen::{
    authenticator_server::Authenticator, LoginRequest, LoginResponse, RegisterRequest,
    RegisterResponse,
};
use shaku::HasComponent;
use tonic::{Code, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyAuthenticator {}

#[tonic::async_trait]
impl Authenticator for MyAuthenticator {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!(
            "Registration request from user with email: {}",
            request.get_ref().email
        );

        // resolve dependencies from the DI container
        let user_repo: &dyn UserRepo = AUTH_MODULE.get().resolve_ref();
        let password_hasher: &dyn MyPasswordHasher = AUTH_MODULE.get().resolve_ref();

        let email = &request.get_ref().email;
        let password = &request.get_ref().password;

        // check if the user already exists
        if let Ok(_) = user_repo.get_by_email(email.clone()).await {
            return Err(Status::new(Code::Unknown, "User already exists"));
        }

        let password_hash = password_hasher.hash_password(password.clone()).await;

        let result = user_repo
            .create(NewUser {
                email: email.to_owned(),
                password_hash,
            })
            .await;

        // Handle errors
        match result {
            Ok(_) => Ok(Response::new(RegisterResponse::default())),
            Err(err) => match err {
                crate::services::user_repos::user_repo::Error::UniqueViolation => {
                    Err(Status::new(Code::Unknown, "User already exists"))
                }
                _ => Err(Status::new(Code::Unknown, "Error while creating the user")),
            },
        }
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!(
            "Login request from user with email: {}",
            request.get_ref().email
        );

        // resolve dependencies from the DI container
        let user_repo: &dyn UserRepo = AUTH_MODULE.get().resolve_ref();
        let password_hasher: &dyn MyPasswordHasher = AUTH_MODULE.get().resolve_ref();
        let token_authenticator: &dyn TokenAuthenticator = AUTH_MODULE.get().resolve_ref();

        let email = &request.get_ref().email;
        let password = &request.get_ref().password;

        // fetch the user from the users repo
        let user = user_repo
            .get_by_email(email.clone())
            .await
            .map_err(|_| Status::new(Code::PermissionDenied, "Wrong user credentials"))?;

        if !password_hasher
            .verify_password(password.to_owned(), user.password_hash.clone())
            .await
        {
            return Err(Status::new(
                Code::PermissionDenied,
                "Wrong user credentials",
            ));
        }

        let access_token = token_authenticator.generate_access_token(user);
        let refresh_token = token_authenticator.generate_refresh_token();

        Ok(Response::new(LoginResponse {
            access_token,
            refresh_token,
        }))
    }
}
