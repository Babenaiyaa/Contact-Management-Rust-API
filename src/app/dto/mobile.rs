use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMobileRequest {
    pub number: String,
}

#[derive(Debug, Serialize)]
pub struct MobileResponse {
    pub id: i32,
    pub person_id: i32,
    pub number: String,
}