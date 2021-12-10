use actix_web::{HttpResponse, Responder, web};
use actix_web::{get};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check_handler);
}

#[get("/alive")]
async fn health_check_handler() -> impl Responder{
    HttpResponse::Ok().body("salut")
}

