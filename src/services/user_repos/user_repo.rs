use shaku::Interface;
use uuid::Uuid;

use crate::models::user::{NewUser, User};

#[tonic::async_trait]
pub trait UserRepo: Interface {
    async fn get_by_id(&self, id: Uuid) -> Result<User, Error>;
    async fn get_by_email(&self, email: String) -> Result<User, Error>;
    async fn create(&self, user: NewUser) -> Result<usize, Error>;
}

#[allow(dead_code)]
pub enum Error {
    NotFound,
    UniqueViolation,
    BackendError,
}
