
use serde::Serialize;
use okapi::schemars::JsonSchema;
use diesel::{Queryable,Insertable};
use crate::services::authentication::LoginForm;
use crate::database::users;

#[derive(Queryable,Insertable,Serialize,JsonSchema)]
pub struct User{
    pub username: String,
    pub password: Option<String>
}

impl From<LoginForm> for User {
    fn from(form: LoginForm) -> Self {
        User{username: form.username, password: Some(form.password)}
    }
}