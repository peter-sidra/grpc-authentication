use crate::schema::users;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}
