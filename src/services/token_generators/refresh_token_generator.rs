use shaku::Interface;

pub trait RefreshTokenGenerator: Interface {
    fn generate_token(&self) -> String;
}
