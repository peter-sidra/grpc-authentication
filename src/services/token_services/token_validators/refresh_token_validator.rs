use shaku::Interface;

pub trait RefreshTokenValidator: Interface {
    fn validate_token(&self, token: &str) -> anyhow::Result<()>;
}
