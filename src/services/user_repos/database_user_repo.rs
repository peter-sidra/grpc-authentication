use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use shaku::Component;
use uuid::Uuid;

use super::user_repo::{Error, UserRepo};
use crate::models::user::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;

#[derive(Component)]
#[shaku(interface = UserRepo)]
pub struct DatabaseUserRepo {
    db_connection_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseUserRepo {
    fn get_db_connection(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.db_connection_pool
            .get()
            .expect("Couldn't aquire a database connection from the pool")
    }

    fn map_diesel_err(&self, diesel_err: diesel::result::Error) -> Error {
        match diesel_err {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Error::UniqueViolation,

                _ => Error::BackendError,
            },
            _ => Error::BackendError,
        }
    }
}

#[tonic::async_trait]
impl UserRepo for DatabaseUserRepo {
    async fn create(&self, new_user: NewUser) -> Result<usize, Error> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            email: new_user.email,
            password_hash: new_user.password_hash,
        };

        let db_conn = &self.get_db_connection();
        diesel::insert_into(users::table)
            .values(&user)
            .execute(db_conn)
            .map_err(|err| self.map_diesel_err(err))
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User, Error> {
        let db_conn = &self.get_db_connection();
        users::table
            .filter(users::id.eq(id.to_string()))
            .first::<User>(db_conn)
            .map_err(|err| self.map_diesel_err(err))
    }

    async fn get_by_email(&self, email: String) -> Result<User, Error> {
        let db_conn = &self.get_db_connection();
        users::table
            .filter(users::email.eq(email))
            .first(db_conn)
            .map_err(|err| self.map_diesel_err(err))
    }
}
