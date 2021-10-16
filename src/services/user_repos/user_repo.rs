use uuid::Uuid;

use crate::models::user::{NewUser, User};

#[tonic::async_trait]
trait UserRepo {
    async fn get_by_id(id: Uuid) -> User;
    async fn get_by_email(email: String) -> User;
    async fn create(user: NewUser);
}
