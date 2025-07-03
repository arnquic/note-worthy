use crate::router::AppState;
use axum::{Json, extract::State, http::StatusCode};
use entity::client::{ActiveModel, Model};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
};
use std::sync::Arc;

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Model>,
) -> Result<(StatusCode, Json<Model>), (StatusCode, Json<serde_json::Value>)> {
    let new_client = ActiveModel {
        id: NotSet,
        date_of_birth: Set(payload.date_of_birth),
        email: Set(payload.email),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        password: Set(payload.password),
        phone: Set(payload.phone),
        preferred_name: Set(payload.preferred_name),
        pronouns: Set(payload.pronouns),
        client_status: Set(payload.client_status),
        created_at: NotSet,
        updated_at: NotSet,
    };

    log::debug!("New client to be inserted: {:?}", new_client);

    let result = new_client.insert(&state.db).await;

    match result {
        Ok(inserted_client) => Ok((StatusCode::CREATED, Json(inserted_client))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"msg": "failed to create user", "error": err.to_string()})),
        )),
    }
}
