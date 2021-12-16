use rocket::{Build, Rocket};
use rocket_cors::AllowedOrigins;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};


use crate::services;

pub fn config_web(mut rocket: Rocket<Build>) -> Rocket<Build>{
    rocket = mount_routes(rocket);
    rocket = mount_ui(rocket);
    attach_cors(rocket)
}


fn mount_routes(mut rocket: Rocket<Build>) -> Rocket<Build> {
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
            rocket,
            "/",
            settings,
            "/auth" => services::authentication::routes(),
            "" => services::health_check::routes(),
            "/postes" => services::postes::routes(),
            "/users" => services::users::routes()
        };
    rocket
}

fn mount_ui(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/ui/", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Spec file", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
}

fn attach_cors(rocket: Rocket<Build>) -> Rocket<Build>{
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    };
    rocket.attach(cors.to_cors().expect("Failed to create CORS policy"))
}
