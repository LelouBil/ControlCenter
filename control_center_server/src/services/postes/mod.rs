use diesel::RunQueryDsl;
use okapi::openapi3::OpenApi;
use rocket::{Route};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec, };
mod data;

use data::Poste;
use crate::database::DatabaseConnection;
use crate::database::postes::dsl::postes;

use crate::services::authentication::LoggedInUser;


pub fn routes() -> (Vec<Route>, OpenApi){
    openapi_get_routes_spec![list_postes]
}

/// Liste les postes trouvés
#[openapi(tag = "Postes")]
#[get("/")]
async fn list_postes(db : DatabaseConnection,_user : LoggedInUser) -> Json<Vec<Poste>>{
    Json(db.run(|conn|{
        postes.load::<Poste>(conn)
            .expect("Erreure de chargement des postes")
    }).await)
}