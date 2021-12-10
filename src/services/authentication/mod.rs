mod data;

use actix_web::{web, post, Result};

pub use data::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        .service(log_in)
    );
}

#[post("/login")]
async fn log_in(login_form: web::Json<User>) -> Result<String>{
    Ok(format!("essai de login avec {} et {}",login_form.name,login_form.password))
}
