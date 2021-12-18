#![feature(drain_filter)]

mod services;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;


mod web_config;
mod database;

#[launch]
fn ignition() -> _ {
    dotenv::dotenv().expect("Failed to load environment");
    println!("Starting rocket launch");
    let rocket = rocket::build()
        .configure(database::get_database_config())
        .attach(database::DatabaseConnection::fairing());
    web_config::config_web(rocket)
}