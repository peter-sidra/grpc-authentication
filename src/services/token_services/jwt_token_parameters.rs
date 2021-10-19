use jwt_simple::prelude::HS256Key;

#[derive(Clone)]
pub struct JwtTokenParameters {
    pub issuer: String,
    pub key: HS256Key,
    pub expiration: u32,
}
