use crate::{
    di::AUTH_MODULE,
    services::token_services::token_validators::access_token_validator::AccessTokenValidator,
};
use shaku::HasComponent;
use tonic::{Request, Status};

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let access_token = match req.metadata().get("access_token") {
        Some(value) => value,
        None => return Err(Status::unauthenticated("No access token provided")),
    }
    .to_str()
    .expect("Error while reading access token from request");

    // Inject the access token validator
    let access_token_validator: &dyn AccessTokenValidator = AUTH_MODULE.get().resolve_ref();

    let claims = match access_token_validator.validate_token(access_token) {
        Ok(value) => value,
        Err(_) => return Err(Status::unauthenticated("Invalid access token")),
    };

    req.extensions_mut().insert(claims);

    Ok(req)
}
