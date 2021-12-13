mod data;
mod security;

use jsonwebtoken::{EncodingKey, Header};
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, openapi_get_routes_spec};

pub use data::*;
use crate::services::users::User;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![log_in]
}

#[openapi(tag = "Login")]
/// Connexion au serveur en utilisant un login/mot de passe
#[post("/login", data = "<login_form>", format = "json")]
pub async fn log_in(login_form: Json<LoginForm>) -> Result<String,Status> {
    let user  = User{username: login_form.username.clone(),password: login_form.password.clone()};

    let result = LoggedInUser::create_jwt(user);
    match result { 
        Some(token) => Ok(token),
        None => Err(Status::InternalServerError)
    }
}
