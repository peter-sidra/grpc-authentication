use crate::models::user::{NewUser, User};
use crate::services::repo_error;
use shaku::Interface;
use uuid::Uuid;

#[tonic::async_trait]
pub trait UserRepo: Interface {
    async fn get_by_id(&self, id: Uuid) -> Result<User, repo_error::Error>;
    async fn get_by_email(&self, email: String) -> Result<User, repo_error::Error>;
    async fn create(&self, user: NewUser) -> Result<usize, repo_error::Error>;
}
