use serde::Serialize;
use schemars::JsonSchema;
use diesel::Queryable;

#[derive(Serialize,JsonSchema,Queryable)]
pub struct Poste{
    pub is_on: bool,
    pub is_compromised: bool,
    pub os: String,
    pub hostname: String,
    pub ip: String,
}