use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct JwtTokenParameters {
    pub issuer: String,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey<'static>,
    pub expiration: u32,
}
