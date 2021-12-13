use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Unauthorized;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use serde::{Deserialize,Serialize};
use schemars::JsonSchema;
use serde_json::json;

use crate::services::users::User;

#[derive(Deserialize,JsonSchema)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LoggedInUser{
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}


static SECRET: &[u8; 5] = b"salut";

static EXP_TIME: i64 = 60 * 60 * 12;
impl LoggedInUser {
    pub fn create_jwt(user: User) -> Option<String> {
        let now = Utc::now().timestamp();
        let user = LoggedInUser{username: user.username, exp: now + EXP_TIME,iat: now };
        jsonwebtoken::encode(&Header::default(), &user, &EncodingKey::from_secret(SECRET)).ok()
    }
    pub fn from_jwt(token: String) -> Option<Self>{
        let result = jsonwebtoken::decode::<LoggedInUser>(&token, &DecodingKey::from_secret(SECRET), &Validation::default());
        result.map(|decoded| decoded.claims).ok()
    }
}


