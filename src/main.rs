mod services;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::http::Method;
use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi_get_routes};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};

use rocket_okapi::settings::{OpenApiSettings, UrlObject};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod web_config;
mod database;


#[rocket::main]
async fn main() {
    let mut rocket = rocket::build();
    rocket = web_config::config_web(rocket);
    database::config_database();
    rocket.launch().await;
}
