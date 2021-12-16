use std::env;

use rocket::figment::{Figment, util::map};
use rocket::figment::value::{Map, Value};
use rocket::Request;
use rocket::request::{FromRequest};
use rocket_okapi::request::{OpenApiFromRequest};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;
mod schema;
use diesel::Connection;

#[derive(OpenApiFromRequest)]
#[database("database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

pub use schema::*;


pub fn get_database_config() -> Figment {
    rocket::Config::figment()
        .merge(("databases", map!{
            "database" => map!{
                "url" => env::var("DATABASE_URL").expect("Database url not found")
             }
        }))
}