use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use shaku::Component;
use uuid::Uuid;

use super::user_repo::UserRepo;
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
}

#[tonic::async_trait]
impl UserRepo for DatabaseUserRepo {
    async fn create(&self, new_user: NewUser) -> Result<usize, diesel::result::Error> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            email: new_user.email,
            password_hash: new_user.password_hash,
        };

        let db_conn = &self.get_db_connection();
        diesel::insert_into(users::table)
            .values(&user)
            .execute(db_conn)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User, diesel::result::Error> {
        let db_conn = &self.get_db_connection();
        users::table
            .filter(users::id.eq(id.to_string()))
            .first::<User>(db_conn)
    }

    async fn get_by_email(&self, email: String) -> Result<User, diesel::result::Error> {
        let db_conn = &self.get_db_connection();
        users::table.filter(users::email.eq(email)).first(db_conn)
    }
}
