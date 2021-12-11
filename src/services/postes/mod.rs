use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::response::status::NoContent;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, openapi_get_routes_spec, };
mod data;

use data::Poste;



pub fn routes() -> (Vec<Route>, OpenApi){
    openapi_get_routes_spec![list_postes]
}

/// Liste les postes trouvés
#[openapi(tag = "Postes")]
#[get("/")]
async fn list_postes() -> Json<Vec<Poste>>{
    let mut random_postes: Vec<Poste> = Vec::new();
    
    let a = Poste{
        ip: "192.168.0.142".to_string(),
        os: "Debian".to_string(),
        hostname: "dinfo142".to_string(),
        is_compromised: false,
        is_on: true
    };
    let b = Poste{
        ip: "192.168.0.198".to_string(),
        os: "Windows".to_string(),
        hostname: "dinfo198".to_string(),
        is_compromised: false,
        is_on: false
    };
    let c = Poste{
        ip: "192.168.0.216".to_string(),
        os: "Windows".to_string(),
        hostname: "dinfo216".to_string(),
        is_compromised: false,
        is_on: true
    };
    let d = Poste{
        ip: "192.168.0.234".to_string(),
        os: "CentOS".to_string(),
        hostname: "dinfo234".to_string(),
        is_compromised: true,
        is_on: true
    };
    random_postes.push(a);
    random_postes.push(b);
    random_postes.push(c);
    random_postes.push(d);

    Json(random_postes)
}