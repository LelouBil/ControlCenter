use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize,JsonSchema)]
pub struct User{
    pub name: String,
    pub password: String
}