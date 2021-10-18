use shaku::Interface;

use crate::models::user::User;

pub trait AccessTokenGenerator: Interface {
    fn generate_token(&self, user: User) -> String;
}
