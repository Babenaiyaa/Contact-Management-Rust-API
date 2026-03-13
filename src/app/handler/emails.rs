use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    app::{
        dto::email::{CreateEmailRequest, EmailResponse},
        state::AppState,
    },
    entities::{emails, persons},
};

pub async fn create_email(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(payload): Json<CreateEmailRequest>,
) -> Result<Json<EmailResponse>, StatusCode> {
    let owner = persons::Entity::find_by_id(person_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if owner.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let new_email = emails::ActiveModel {
        person_id: Set(person_id),
        email: Set(payload.email),
        ..Default::default()
    };

    let result = new_email
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(EmailResponse {
        id: result.id,
        person_id: result.person_id,
        email: result.email,
    }))
}

pub async fn list_emails(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<Vec<EmailResponse>>, StatusCode> {
    let items = emails::Entity::find()
        .filter(emails::Column::PersonId.eq(person_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = items
        .into_iter()
        .map(|item| EmailResponse {
            id: item.id,
            person_id: item.person_id,
            email: item.email,
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let item = emails::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(item) = item else {
        return Err(StatusCode::NOT_FOUND);
    };

    let active_model: emails::ActiveModel = item.into();

    active_model
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}