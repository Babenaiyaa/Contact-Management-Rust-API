use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct EmailResponse {
    pub id: i32,
    pub person_id: i32,
    pub email: String,
}