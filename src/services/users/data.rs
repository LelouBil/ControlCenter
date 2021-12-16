

use diesel::{Queryable,Insertable};
use crate::services::authentication::LoginForm;

#[derive(Queryable)]
pub struct User{
    pub username: String,
    pub password: String
}

impl From<LoginForm> for User {
    fn from(form: LoginForm) -> Self {
        User{username: form.username, password: form.password}
    }
}