use super::refresh_token_repo::RefreshTokenRepo;
use crate::connection_pool_wrapper::DBConnectionPool;
use crate::models::refresh_token::{NewRefreshToken, RefreshToken};
use crate::schema;
use crate::services::repo_error;
use diesel::prelude::*;
use schema::refresh_tokens;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = RefreshTokenRepo)]
pub struct DatabaseRefreshTokenRepo {
    #[shaku(inject)]
    db_connection_pool: Arc<DBConnectionPool>,
}

#[tonic::async_trait]
impl RefreshTokenRepo for DatabaseRefreshTokenRepo {
    async fn create(&self, new_refresh_token: NewRefreshToken) -> Result<usize, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let token = RefreshToken {
                id: Uuid::new_v4().to_string(),
                token: new_refresh_token.token,
                user_id: new_refresh_token.user_id,
            };

            let db_conn = db_connection_pool.get();

            diesel::insert_into(refresh_tokens::table)
                .values(&token)
                .execute(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        match task.await {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error while awaiting database_refresh_token_repo's create");
                Err(repo_error::Error::BackendError)
            }
        }
    }

    async fn get_by_token(&self, token: String) -> Result<RefreshToken, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let db_conn = db_connection_pool.get();

            refresh_tokens::table
                .filter(refresh_tokens::token.eq(token))
                .first::<RefreshToken>(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        match task.await {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error while awaiting database_refresh_token_repo's get_by_token");
                Err(repo_error::Error::BackendError)
            }
        }
    }

    async fn delete(&self, id: String) -> Result<usize, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let db_conn = db_connection_pool.get();

            diesel::delete(refresh_tokens::table)
                .filter(refresh_tokens::id.eq(id))
                .execute(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        match task.await {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error while awaiting database_refresh_token_repo's delete");
                Err(repo_error::Error::BackendError)
            }
        }
    }

    async fn delete_all_by_user_id(&self, user_id: String) -> Result<usize, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let db_conn = db_connection_pool.get();

            diesel::delete(refresh_tokens::table)
                .filter(refresh_tokens::user_id.eq(user_id))
                .execute(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        match task.await {
            Ok(value) => value,
            Err(_) => {
                eprintln!(
                    "Error while awaiting database_refresh_token_repo's delete_all_by_user_id"
                );
                Err(repo_error::Error::BackendError)
            }
        }
    }
}
