mod data;
mod security;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{QueryDsl, RunQueryDsl};

use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::services::users::User;
use crate::web_config::{DieselErrorToRocketStatus};


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![log_in]
}

#[openapi(tag = "Login")]
/// Connexion au serveur en utilisant un login/mot de passe
#[post("/login", data = "<login_form>", format = "json")]
pub async fn log_in(db: DatabaseConnection, login_form: Json<LoginForm>) -> Result<String,Status> {
    let username = login_form.username.clone();
    let user : User = db.run(move |conn| {
        users.find(username).first::<User>(conn)
            .map_err_to_status()
            .map_err(|error| if error == Status::NotFound {Status::Unauthorized} else { error } )
    }).await?;
    match &user.password {
        Some(pass) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(pass)
                .map_err(|e| {
                    eprintln!("Error parsing hash : {}",e);
                    Status::Unauthorized
                })?;
            argon2.verify_password(login_form.password.as_bytes(),&parsed_hash)
                .map_err(|e| Status::Unauthorized)?;
            LoggedInUser::create_jwt(user).ok_or(Status::InternalServerError)
        }
        None => {
            Err(Status::Unauthorized)
        }
        _ => unreachable!()
    }
}
