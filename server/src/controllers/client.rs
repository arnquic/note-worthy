use crate::router::AppState;
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use entity::client;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<client::Model>,
) -> Result<(StatusCode, Json<client::Model>), (StatusCode, Json<serde_json::Value>)> {
    let hashed_password = hash(payload.password, DEFAULT_COST).unwrap();

    let new_client = client::ActiveModel {
        id: NotSet,
        client_status: Set(payload.client_status),
        date_of_birth: Set(payload.date_of_birth),
        email: Set(payload.email),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        password: Set(hashed_password),
        phone: Set(payload.phone),
        preferred_name: Set(payload.preferred_name),
        pronouns: Set(payload.pronouns),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSigninRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSigninResponse {
    jwt: String,
}

pub async fn signin(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ClientSigninRequest>,
) -> Result<(StatusCode, Json<ClientSigninResponse>), (StatusCode, Json<serde_json::Value>)> {
    let client_result = client::Entity::find()
        .filter(client::Column::Email.eq(payload.email))
        .all(&state.db)
        .await;

    if let Ok(client_hits) = client_result {
        if client_hits.is_empty() {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"msg": "client not found"})),
            ));
        } else if client_hits.len() > 1 {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"msg": "multiple clients found"})),
            ));
        } else {
            let client = client_hits.first().unwrap();
            log::debug!("Single user found: {:?}", client.email);

            let verify_result = verify(payload.password, &client.password);
            log::debug!("Verification result: {:?}", verify_result);

            match verify_result {
                Ok(true) => {
                    log::debug!("Successfully verified");
                    return Ok((
                        StatusCode::OK,
                        Json(ClientSigninResponse {
                            jwt: "token here".to_string(),
                        }),
                    ));
                }
                Ok(false) | Err(_) => {
                    log::debug!("Failed to verify password");
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({"msg": "invalid credentials"})),
                    ));
                }
            }
        }
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"msg": "database error"})),
        ))
    }
}
