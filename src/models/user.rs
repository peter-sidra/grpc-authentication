use crate::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}
