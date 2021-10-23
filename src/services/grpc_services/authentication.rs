pub mod proto_gen {
    tonic::include_proto!("authentication"); // The string specified here must match the proto package name
}

use crate::{
    di::AUTH_MODULE,
    models::{refresh_token::NewRefreshToken, user::NewUser},
    services::{
        password_hashers::hasher::PasswordHasher as MyPasswordHasher,
        refresh_token_repos::refresh_token_repo::RefreshTokenRepo,
        token_services::{
            token_generators::refresh_token_generator::RefreshTokenGenerator,
            token_validators::{
                access_token_validator::AccessTokenValidator,
                refresh_token_validator::RefreshTokenValidator,
            },
        },
    },
    services::{
        token_services::token_authenticator::TokenAuthenticator, user_repos::user_repo::UserRepo,
    },
};
use proto_gen::{
    authenticator_server::Authenticator, LoginRequest, LoginResponse, LogoutRequest,
    LogoutResponse, RefreshRequest, RefreshResponse, RegisterRequest, RegisterResponse,
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
        if user_repo.get_by_email(email.clone()).await.is_ok() {
            return Err(Status::new(Code::AlreadyExists, "User already exists"));
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
                crate::services::repo_error::Error::UniqueViolation => {
                    Err(Status::new(Code::AlreadyExists, "User already exists"))
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

        // Create and store the refresh token
        let refresh_token = token_authenticator.generate_refresh_token();
        let refresh_token_repo: &dyn RefreshTokenRepo = AUTH_MODULE.get().resolve_ref();

        if refresh_token_repo
            .create(NewRefreshToken {
                token: refresh_token.clone(),
                user_id: user.id.clone(),
            })
            .await
            .is_err()
        {
            return Err(Status::unknown(""));
        }

        // Create the access token
        let access_token = token_authenticator.generate_access_token(user);

        Ok(Response::new(LoginResponse {
            access_token,
            refresh_token,
        }))
    }

    // This invalidates all of the refresh tokens belonging to the user
    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let access_token = &request.get_ref().access_token;

        let access_token_validator: &dyn AccessTokenValidator = AUTH_MODULE.get().resolve_ref();

        let validation_result = access_token_validator.validate_token(access_token);

        let claims = match validation_result {
            Ok(value) => value,
            Err(_) => return Err(Status::permission_denied("")),
        };

        let user_id = claims.id;

        let refresh_token_repo: &dyn RefreshTokenRepo = AUTH_MODULE.get().resolve_ref();

        if refresh_token_repo
            .delete_all_by_user_id(user_id)
            .await
            .is_err()
        {
            return Err(Status::unknown(""));
        }

        Ok(Response::new(LogoutResponse {}))
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let refresh_token_validator: &dyn RefreshTokenValidator = AUTH_MODULE.get().resolve_ref();
        let refresh_token_repo: &dyn RefreshTokenRepo = AUTH_MODULE.get().resolve_ref();
        let refresh_token_generator: &dyn RefreshTokenGenerator = AUTH_MODULE.get().resolve_ref();

        let request_refresh_token = &request.get_ref().refresh_token;

        if refresh_token_validator
            .validate_token(request_refresh_token)
            .is_err()
        {
            return Err(Status::permission_denied("Invalid refresh token"));
        };

        let db_refresh_token = match refresh_token_repo
            .get_by_token(request_refresh_token.to_owned())
            .await
        {
            Ok(token) => token,
            Err(_) => return Err(Status::permission_denied("Invalid refresh token")),
        };

        if refresh_token_repo
            .delete(db_refresh_token.id)
            .await
            .is_err()
        {
            return Err(Status::internal(""));
        }

        let new_refresh_token = refresh_token_generator.generate_token();
        if refresh_token_repo
            .create(NewRefreshToken {
                token: new_refresh_token.clone(),
                user_id: db_refresh_token.user_id,
            })
            .await
            .is_err()
        {
            return Err(Status::internal(""));
        }

        Ok(Response::new(RefreshResponse {
            refresh_token: new_refresh_token,
        }))
    }
}
