use okapi::openapi3::OpenApi;
use rocket::{Route, routes as rocket_routes};
use rocket_okapi::{openapi, openapi_get_routes_spec};


pub fn routes() -> (Vec<Route>, OpenApi){
    openapi_get_routes_spec![health_check]
}

/// Verifie si l'application est active

#[openapi]
#[get("/alive")]
async fn health_check() -> &'static str{
    "salut"
}

