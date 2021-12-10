mod data;

use actix_web::{web, get, Responder};
use actix_web::web::Json;


pub use data::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postes")
            .service(list_postes)
    );
}


#[get("")]
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