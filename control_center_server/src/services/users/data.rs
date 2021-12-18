
use serde::{Serialize,Deserialize};
use okapi::schemars::JsonSchema;
use diesel::{Queryable,Insertable};
use okapi::map;
use okapi::openapi3::{RefOr, Response, Responses};
use rocket::Request;
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::{OpenApiResponder, OpenApiResponderInner};
use crate::services::authentication::LoginForm;
use crate::database::users;

#[derive(Queryable,Insertable,Serialize,JsonSchema)]
pub struct User{
    pub username: String,
    pub password: Option<String>
}

#[derive(Deserialize,JsonSchema)]
pub struct UserPasswordForm{
    pub password: Option<String>
}

impl From<LoginForm> for User {
    fn from(form: LoginForm) -> Self {
        User{username: form.username, password: Some(form.password)}
    }
}