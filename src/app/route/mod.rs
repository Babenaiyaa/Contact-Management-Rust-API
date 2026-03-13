use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::app::{
    handler::{
        emails::{create_email, delete_email, list_emails},
        mobiles::{create_mobile, delete_mobile, list_mobiles},
        persons::{create_person, delete_person, get_person, list_persons, update_person},
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/api/v1/persons", post(create_person))
        .route("/api/v1/persons", get(list_persons))
        .route("/api/v1/persons/:id", get(get_person))
        .route("/api/v1/persons/:id", put(update_person))
        .route("/api/v1/persons/:id", delete(delete_person))
        .route("/api/v1/persons/:id/emails", post(create_email))
        .route("/api/v1/persons/:id/emails", get(list_emails))
        .route("/api/v1/emails/:id", delete(delete_email))
        .route("/api/v1/persons/:id/mobiles", post(create_mobile))
        .route("/api/v1/persons/:id/mobiles", get(list_mobiles))
        .route("/api/v1/mobiles/:id", delete(delete_mobile))
}

async fn root() -> &'static str {
    "Contact Management Service with PostgreSQL + SeaORM"
}