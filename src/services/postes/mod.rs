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
        ip: "192.168.0.243".to_string(),
        os: "CentOS".to_string(), 
        hostname: "dinfo243".to_string(),
        is_compromised: true, 
        is_on: true
    };
    let b = Poste{
        ip: "192.168.0.123".to_string(),
        os: "CentOS".to_string(),
        hostname: "dinfo123".to_string(),
        is_compromised: true,
        is_on: true
    };
    random_postes.push(a);
    random_postes.push(b);
    
    Json(random_postes)
}