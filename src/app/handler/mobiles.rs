use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    app::{
        dto::mobile::{CreateMobileRequest, MobileResponse},
        state::AppState,
    },
    entities::{mobiles, persons},
};

pub async fn create_mobile(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(payload): Json<CreateMobileRequest>,
) -> Result<Json<MobileResponse>, StatusCode> {
    let owner = persons::Entity::find_by_id(person_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if owner.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let new_mobile = mobiles::ActiveModel {
        person_id: Set(person_id),
        number: Set(payload.number),
        ..Default::default()
    };

    let result = new_mobile
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MobileResponse {
        id: result.id,
        person_id: result.person_id,
        number: result.number,
    }))
}

pub async fn list_mobiles(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<Vec<MobileResponse>>, StatusCode> {
    let items = mobiles::Entity::find()
        .filter(mobiles::Column::PersonId.eq(person_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = items
        .into_iter()
        .map(|item| MobileResponse {
            id: item.id,
            person_id: item.person_id,
            number: item.number,
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let item = mobiles::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(item) = item else {
        return Err(StatusCode::NOT_FOUND);
    };

    let active_model: mobiles::ActiveModel = item.into();

    active_model
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}