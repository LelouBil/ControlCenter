mod services;

#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi_get_routes};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};

use rocket_okapi::settings::{OpenApiSettings, UrlObject};

use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[rocket::main]
async fn main() {
    let mut rocket = rocket::build();
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    };
    {
        let settings = OpenApiSettings::default();
        mount_endpoints_and_merged_docs! {
            rocket,
            "/",
            settings,
            "/auth" => services::authentication::routes(),
            "/" => services::health_check::routes(),
            "/postes" => services::postes::routes(),
        }
        ;
    }

    rocket
        .mount("/ui/", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Spec file", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
        .attach(cors.to_cors().unwrap())
        .launch()
        .await;
}
