mod data;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use diesel::{delete, ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, update};
use okapi::openapi3::{OpenApi, PathItem, RefOr};
use okapi::openapi3::RefOr::Object;
use rocket::response::status;
use rocket::response::status::Conflict;
use rocket::Route;
use rocket::serde::json::Json;
use rocket_okapi::{openapi_get_routes_spec,openapi};
pub use data::*;
use crate::database::DatabaseConnection;
use crate::database::users::dsl::users;
use crate::database::users::{password, username};
use crate::web_config::{DatabaseErrors, DieselErrorToRocketStatus};
use crate::services::authentication::{LoggedInUser, LoginForm};


pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![create_user,get_user,list_users,change_password]
}

/// Créé un utilisateur
#[openapi(tag = "Users")]
#[post("/", data = "<login_form>", format = "json")]
pub async fn create_user(db: DatabaseConnection,_user : LoggedInUser, login_form: Json<LoginForm>) -> Result<status::Created<Json<User>>,status::Conflict<()>> {
    let mut new_user = User::from(login_form.into_inner());
    
    if let Some(pass) = new_user.password{
        let hashed_pass = hash_password(pass);
        new_user.password = Some(hashed_pass.expect("Failed to hash password"));
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

fn hash_password(pass: String) -> Result<String,argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2.hash_password(pass.as_bytes(), &salt)
        .map(|pass| pass.to_string())
}

/// Liste les utilisateurs
#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(db: DatabaseConnection,_user : LoggedInUser) -> Json<Vec<User>>{
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
pub async fn get_user(db: DatabaseConnection,_user : LoggedInUser,user_name: String) -> Option<Json<User>>{
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


///Change le mot de passe d'un utilisateur
#[openapi(tag = "Users")]
#[post("/@me", data = "<new_password>", format = "json")]
pub async fn change_password(db: DatabaseConnection,_user : LoggedInUser,new_password: Json<UserPasswordForm>) -> Result<(),rocket::response::status::NotFound<()>>{
    let name = _user.username.clone();
    let affected_rows = db.run(move |conn| {
        update(users.filter(username.eq(_user.username)))
            .set(password.eq(new_password.into_inner().password.map(|pass| {
                    hash_password(pass).expect("Failed to hash password")
                })
            ))
            .execute(conn)
            .expect("Failed to edit password")
    }).await;

    match affected_rows {
        1 => Ok(()),
        0 => Err(rocket::response::status::NotFound(())),
        _ => panic!("Changing password of {} affected multiple users !",name)
    }
}