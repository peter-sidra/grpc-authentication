use crate::models::refresh_token::{NewRefreshToken, RefreshToken};
use crate::services::repo_error;
use shaku::Interface;

#[tonic::async_trait]
pub trait RefreshTokenRepo: Interface {
    async fn create(&self, new_refresh_token: NewRefreshToken) -> Result<usize, repo_error::Error>;
    async fn get_by_token(&self, token: String) -> Result<RefreshToken, repo_error::Error>;
    async fn delete(&self, id: String) -> Result<usize, repo_error::Error>;
    async fn delete_all_by_user_id(&self, user_id: String) -> Result<usize, repo_error::Error>;
}
