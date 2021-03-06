mod data;
mod security;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{QueryDsl, RunQueryDsl};

use okapi::openapi3::OpenApi;
use rocket::response::status::Unauthorized;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::services::users::User;
use crate::web_config::{DatabaseErrors, DieselErrorToRocketStatus};


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![log_in]
}

#[openapi(tag = "Login")]
/// Connexion au serveur en utilisant un login/mot de passe
#[post("/login", data = "<login_form>", format = "json")]
pub async fn log_in(db: DatabaseConnection, login_form: Json<LoginForm>) -> Result<String,rocket::response::status::Unauthorized<()>> {
    let username = login_form.username.clone();
    let user : User = db.run(move |conn| {
        users.find(username).first::<User>(conn)
            .allow_to(DatabaseErrors::NotFound, Unauthorized(None))
    }).await?;
    match &user.password {
        Some(pass) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(pass)
                .expect("Error parsing hash");
            argon2.verify_password(login_form.password.as_bytes(),&parsed_hash)
                .map_err(|_|Unauthorized(None))?;
            Ok(LoggedInUser::create_jwt(user).expect("Failed to create JWT"))
        }
        None => {
            Err(Unauthorized(None))
        }
    }
}
