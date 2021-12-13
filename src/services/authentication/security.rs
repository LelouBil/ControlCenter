use okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use crate::services::authentication::LoggedInUser;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let headers = request.headers();
        let authh = headers.get_one("Authorization");
        if authh.is_none(){
            return Outcome::Failure((Status::Unauthorized,()));
        }

        let token = authh.unwrap().replace("Bearer ","");
        println!("Token is {}",token);

        match LoggedInUser::from_jwt(token) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Failure((Status::Unauthorized,()))
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for LoggedInUser {
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        let scheme = SecurityScheme{
            description: Some("Cette action necesitte d'etre connécté".into()),
            data: SecuritySchemeData::Http{
                scheme: "Bearer".into(),
                bearer_format: Some("JWT".into())
            },
            extensions: Object::default()
        };

        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("JWT".into(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "JWT".into(),
            scheme,
            security_req,
        ))
    }
}