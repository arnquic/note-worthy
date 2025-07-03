use crate::{NW_HASH_COST, authentication::jwt, router::AppState};
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{hash, verify};
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
    let query_result = client::Entity::find()
        .filter(client::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await;

    if let Ok(client_option) = query_result {
        if let Some(_client) = client_option {
            return Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"msg": "Client already exists"})),
            ));
        }
    }

    let hashed_password = hash(payload.password, NW_HASH_COST).unwrap();

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
            Json(serde_json::json!({"msg": "failed to create client", "error": err.to_string()})),
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
        match client_hits.len() {
            0 => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"msg": "client not found"})),
                ));
            }
            1 => {
                let client = client_hits.first().unwrap();
                log::debug!("Single user found: {:?}", client.email);

                let verify_result = verify(payload.password, &client.password);
                log::debug!("Verification result: {:?}", verify_result);

                match verify_result {
                    Ok(true) => {
                        log::debug!("Successfully verified");
                        let jwt_result = jwt::generate(&client.id.to_string(), &client.email, None);

                        if let Ok(jwt) = jwt_result {
                            return Ok((StatusCode::OK, Json(ClientSigninResponse { jwt })));
                        } else {
                            log::error!("Failed to generate JWT");
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({"msg": "failed to generate JWT"})),
                            ));
                        }
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
            _ => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"msg": "multiple clients found"})),
                ));
            }
        }
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"msg": "database error"})),
        ))
    }
}
