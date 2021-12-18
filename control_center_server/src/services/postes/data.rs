use std::ops::{Range, RangeInclusive};
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;
use diesel::Queryable;
use rocket::form::FromForm;

#[derive(Serialize,JsonSchema,Queryable)]
pub struct Poste{
    pub is_on: bool,
    pub is_compromised: bool,
    pub os: String,
    pub hostname: String,
    pub ip: String,
}

#[derive(Deserialize,JsonSchema,FromForm)]
pub struct PostesFilter{
    pub is_on: Option<bool>,
    pub is_compromised: Option<bool>,
    pub os: Option<String>,
    pub ip_range_start: Option<u64>,
    pub ip_range_end: Option<u64>,
}
