mod services;

#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi_get_routes};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};

use rocket_okapi::settings::{OpenApiSettings, UrlObject};

#[rocket::main]
async fn main() {
    let mut rocket = rocket::build();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
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
        .attach(cors.to_cors().unwrap())
        .mount("/ui/", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Spec file", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
        .launch()
        .await;
}
