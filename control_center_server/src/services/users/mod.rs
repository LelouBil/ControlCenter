mod data;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use diesel::{insert_into, QueryDsl, RunQueryDsl};
use okapi::openapi3::OpenApi;
use rocket::response::status;
use rocket::response::status::Conflict;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi_get_routes_spec,openapi};
pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::web_config::{DatabaseErrors, DieselErrorToRocketStatus};
use crate::services::authentication::LoginForm;


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![create_user,get_user,list_users]
}

/// Créé un utilisateur
#[openapi(tag = "Users")]
#[post("/", data = "<login_form>", format = "json")]
pub async fn create_user(db: DatabaseConnection, login_form: Json<LoginForm>) -> Result<status::Created<Json<User>>,status::Conflict<()>> {
    let mut new_user = User::from(login_form.into_inner());
    
    if let Some(pass) = new_user.password{
        let salt = SaltString::generate(&mut OsRng);
        let  argon2 = Argon2::default();
        let hashed_pass = argon2.hash_password(pass.as_bytes(),&salt)
            .expect("Failed to hash password")
            .to_string();
        new_user.password = Some(hashed_pass);
    }
    db.run(move |conn| {
        insert_into(users).values(&new_user).execute(conn)
            .map(move |_| {
                status::Created::new(
                    uri!("/users",get_user(user_name = new_user.username.clone())).to_string())
                    .body(Json(new_user))
            })
            .allow_to(DatabaseErrors::UniqueViolation, Conflict(None))
    }).await
}
/// Liste les utilisateurs
#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(db: DatabaseConnection) -> Json<Vec<User>>{
    let mut userlist : Vec<User>  = db.run(|conn| {
        users.load::<User>(conn)
            .expect("Failed to load users from database")
    }).await;

    for user in &mut userlist {
        user.password = None;
    }
    Json(userlist)
}

///Récupere un utilisateur par son nom d'utilisateur
#[openapi(tag = "Users")]
#[get("/<user_name>")]
pub async fn get_user(db: DatabaseConnection,user_name: String) -> Option<Json<User>>{
    db.run(|conn| {
        users.find(user_name)
            .first::<User>(conn)
            .allow_to(DatabaseErrors::NotFound, ()).ok()
            .map(|mut user| {
                user.password = None;
                Json(user)
            })
    }).await
}
