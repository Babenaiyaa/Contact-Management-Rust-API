use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::{
    app::{
        dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest},
        state::AppState,
    },
    entities::persons,
};

pub async fn create_person(
    State(state): State<AppState>,
    Json(payload): Json<CreatePersonRequest>,
) -> Result<Json<PersonResponse>, StatusCode> {
    let new_person = persons::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };

    let result = new_person
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PersonResponse {
        id: result.id,
        name: result.name,
    }))
}

pub async fn list_persons(
    State(state): State<AppState>,
) -> Result<Json<Vec<PersonResponse>>, StatusCode> {
    let persons_list = persons::Entity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = persons_list
        .into_iter()
        .map(|person| PersonResponse {
            id: person.id,
            name: person.name,
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PersonResponse>, StatusCode> {
    let person = persons::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match person {
        Some(person) => Ok(Json(PersonResponse {
            id: person.id,
            name: person.name,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePersonRequest>,
) -> Result<Json<PersonResponse>, StatusCode> {
    let person = persons::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(person) = person else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut active_model: persons::ActiveModel = person.into();
    active_model.name = Set(payload.name);

    let updated = active_model
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PersonResponse {
        id: updated.id,
        name: updated.name,
    }))
}

pub async fn delete_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let person = persons::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(person) = person else {
        return Err(StatusCode::NOT_FOUND);
    };

    let active_model: persons::ActiveModel = person.into();

    active_model
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
