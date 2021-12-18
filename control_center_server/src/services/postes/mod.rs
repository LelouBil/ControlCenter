use diesel::query_builder::{AsQuery, Query, SelectStatement};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use okapi::openapi3::OpenApi;
use rocket::{Route};
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

mod data;

use data::Poste;
use crate::database::DatabaseConnection;
use crate::database::postes::dsl::postes;
use crate::database::postes::{ip, is_on, os};

use crate::services::authentication::LoggedInUser;
use crate::services::postes::data::PostesFilter;


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![list_postes]
}

/// Liste les postes trouvés
#[openapi(tag = "Postes")]
#[get("/?<filtre..>")]
async fn list_postes(db: DatabaseConnection, _user: LoggedInUser, filtre: PostesFilter) -> Json<Vec<Poste>> {
    let mut postlist: Vec<Poste> = db.run(move |conn| {
        let mut query = postes.as_query().into_boxed();

        if let Some(value) = filtre.is_on {
            query = query.filter(is_on.eq(value))
        }

        if let Some(value) = filtre.is_compromised {
            query = query.filter(is_on.eq(value))
        }

        if let Some(value) = filtre.os {
            query = query.filter(os.eq(value))
        }
        query.load::<Poste>(conn)
            .expect("Erreure de chargement des postes")
    }).await;

    if let Some(value) = filtre.ip_range_start {
        postlist.drain_filter(|poste| {
            poste.ip.split(".").nth(3)
                .and_then(|lastpart| {
                    lastpart.parse::<u64>().ok()
                })
                .map_or(false, |num| value > num)
        });
    }
    if let Some(value) = filtre.ip_range_end {
        postlist.drain_filter(|poste| {
            poste.ip.split(".").nth(3)
                .and_then(|lastpart| lastpart.parse::<u64>().ok())
                .map_or(false, |num| value < num)
        });
    }
    
    Json(postlist)
}