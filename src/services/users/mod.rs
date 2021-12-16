mod data;

use diesel::{insert_into, QueryDsl, RunQueryDsl};
use diesel::result::{DatabaseErrorKind, Error};
use okapi::openapi3::OpenApi;
use rocket::http::Status;
use rocket::http::uri::{Origin, Uri};
use rocket::response::status;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi_get_routes_spec,openapi};
pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::database::users::username;
use crate::services::authentication::LoginForm;


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![create_user,get_user,list_users]
}

/// Créé un utilisateur
#[openapi(tag = "Users")]
#[post("/", data = "<login_form>", format = "json")]
pub async fn create_user(db: DatabaseConnection, login_form: Json<LoginForm>) -> Result<status::Created<Json<User>>,Status> {
    let new_user = User::from(login_form.into_inner());
    db.run(move |conn| {
        insert_into(users).values(&new_user).execute(conn)
            .map(move |_| {
                status::Created::new(
                    uri!("/users",get_user(user_name = new_user.username.clone())).to_string())
                    .body(Json(new_user))
            })
            .map_err(|error| {
                if let diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation,_) = error 
                { Status::Conflict } 
                else { Status::InternalServerError } //todo error catcher to just propagate and REMOVE Status as returns
            })
    }).await
}
/// Liste les utilisateurs
#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(db: DatabaseConnection) -> Result<Json<Vec<User>>,Status>{
    let mut userlist : Vec<User>  = db.run(|conn| {
        users.load::<User>(conn)
            .map_err(|_| Status::InternalServerError)
    }).await?;

    for user in &mut userlist {
        user.password = None;
    }
    Ok(Json(userlist))
}

///Récupere un utilisateur par son nom d'utilisateur
#[openapi(tag = "Users")]
#[get("/<user_name>")]
pub async fn get_user(db: DatabaseConnection,user_name: String) -> Result<Json<User>,Status>{
    db.run(|conn| {
        users.find(user_name)
            .first::<User>(conn)
            .map(|mut user| {
                user.password = None;
                Json(user)
            })
            .map_err(|error| {
                if error == Error::NotFound
                { Status::NotFound }
                else { Status::InternalServerError } 
            })
    }).await
}
