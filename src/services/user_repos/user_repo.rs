use shaku::Interface;
use uuid::Uuid;

use crate::models::user::{NewUser, User};

#[tonic::async_trait]
pub trait UserRepo: Interface {
    async fn get_by_id(&self, id: Uuid) -> Result<User, diesel::result::Error>;
    async fn get_by_email(&self, email: String) -> Result<User, diesel::result::Error>;
    async fn create(&self, user: NewUser) -> Result<usize, diesel::result::Error>;
}
