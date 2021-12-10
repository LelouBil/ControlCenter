mod services;

use actix_web::{App, middleware::Logger,middleware::NormalizePath, HttpServer};
use actix_web::middleware::normalize::TrailingSlash;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .configure(services::health_check::config)
            .configure(services::authentication::config)
            .configure(services::postes::config)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
