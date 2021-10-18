use shaku::Interface;

#[tonic::async_trait]
pub trait PasswordHasher: Interface {
    async fn hash_password(&'static self, password: String) -> String;

    async fn verify_password(&self, password: String, hash: String) -> bool;
}
