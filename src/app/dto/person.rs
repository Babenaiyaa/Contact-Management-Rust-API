use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PersonResponse {
    pub id: i32,
    pub name: String,
}