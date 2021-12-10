mod data;

use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, openapi_get_routes_spec};

pub use data::*;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![log_in]
}

#[openapi]
/// Connexion au serveur en utilisant un login/mot de passe
#[post("/login", data = "<login_form>", format = "json")]
pub async fn log_in(login_form: Json<User>) -> String {
    format!("essai de login avec {} et {}", login_form.name, login_form.password)
}
