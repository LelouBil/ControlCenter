mod data;
mod security;

use std::borrow::Borrow;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::result::Error;
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::services::users::User;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![log_in]
}

#[openapi(tag = "Login")]
/// Connexion au serveur en utilisant un login/mot de passe
#[post("/login", data = "<login_form>", format = "json")]
pub async fn log_in(db: DatabaseConnection, login_form: Json<LoginForm>) -> Result<String,Status> {
    let res : User = db.run(move |conn| {
        match users.find(&login_form.username).first::<User>(conn) {
            Ok(user) => 
                if user.password.is_some() && user.password.as_ref().unwrap() == &login_form.password { Ok(user) } 
                else { Err(Status::Unauthorized) }
            Err(error) => 
                if error == Error::NotFound { Err(Status::Unauthorized) }
                else { Err(Status::InternalServerError) }
        }
    }).await?;

    LoggedInUser::create_jwt(res).ok_or(Status::InternalServerError)
}
