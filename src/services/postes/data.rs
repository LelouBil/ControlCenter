use serde::Serialize;

#[derive(Serialize)]
pub struct Poste{
    pub is_on: bool,
    pub is_compromised: bool,
    pub os: String,
    pub hostname: String,
    pub ip: String,
}