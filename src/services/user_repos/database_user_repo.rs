use super::user_repo::UserRepo;
use crate::connection_pool_wrapper::DBConnectionPool;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use crate::services::repo_error;
use diesel::prelude::*;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component, Clone)]
#[shaku(interface = UserRepo)]
pub struct DatabaseUserRepo {
    #[shaku(inject)]
    db_connection_pool: Arc<DBConnectionPool>,
}

#[tonic::async_trait]
impl UserRepo for DatabaseUserRepo {
    async fn create(&self, new_user: NewUser) -> Result<usize, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let user = User {
                id: Uuid::new_v4().to_string(),
                email: new_user.email,
                password_hash: new_user.password_hash,
            };

            let db_conn = db_connection_pool.get();

            diesel::insert_into(users::table)
                .values(&user)
                .execute(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        task.await
            .expect("Error while awaiting database_user_repo's create")
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let db_conn = db_connection_pool.get();

            users::table
                .filter(users::id.eq(id.to_string()))
                .first::<User>(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        task.await
            .expect("Error while awaiting database_user_repo's get_by_id")
    }

    async fn get_by_email(&self, email: String) -> Result<User, repo_error::Error> {
        let db_connection_pool = self.db_connection_pool.clone();

        let task = tokio::task::spawn_blocking(move || {
            let db_conn = db_connection_pool.get();

            users::table
                .filter(users::email.eq(email))
                .first(&db_conn)
                .map_err(|err| repo_error::Error::map_diesel_err(err))
        });

        task.await
            .expect("Error while awaiting database_user_repo's get_by_email")
    }
}
