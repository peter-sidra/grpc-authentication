use crate::schema::refresh_tokens;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "refresh_tokens"]
pub struct RefreshToken {
    pub id: String,
    pub token: String,
    pub user_id: String,
}

pub struct NewRefreshToken {
    pub token: String,
    pub user_id: String,
}
