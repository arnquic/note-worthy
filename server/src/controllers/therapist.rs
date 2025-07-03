use crate::{NW_HASH_COST, authentication::jwt, router::AppState};
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{hash, verify};
use entity::therapist;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<therapist::Model>,
) -> Result<(StatusCode, Json<therapist::Model>), (StatusCode, Json<serde_json::Value>)> {
    let query_result = therapist::Entity::find()
        .filter(therapist::Column::Email.eq(&payload.email))
        .one(&state.db)
        .await;

    if let Ok(therapist_option) = query_result {
        if let Some(_therapist) = therapist_option {
            return Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"msg": "Therapist already exists"})),
            ));
        }
    }

    let hashed_password = hash(payload.password, NW_HASH_COST).unwrap();

    let new_therapist = therapist::ActiveModel {
        id: NotSet,
        date_of_birth: Set(payload.date_of_birth),
        email: Set(payload.email),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        license_number: Set(payload.license_number),
        password: Set(hashed_password),
        phone: Set(payload.phone),
        role: Set(payload.role),
        specializations: Set(payload.specializations),
        therapist_status: Set(payload.therapist_status),
        created_at: NotSet,
        updated_at: NotSet,
    };

    log::debug!("New therapist to be inserted: {:?}", new_therapist);

    let result = new_therapist.insert(&state.db).await;

    match result {
        Ok(inserted_therapist) => Ok((StatusCode::CREATED, Json(inserted_therapist))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({"msg": "failed to create therapist", "error": err.to_string()}),
            ),
        )),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TherapistSigninRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TherapistSigninResponse {
    jwt: String,
}

pub async fn signin(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TherapistSigninRequest>,
) -> Result<(StatusCode, Json<TherapistSigninResponse>), (StatusCode, Json<serde_json::Value>)> {
    let query_result = therapist::Entity::find()
        .filter(therapist::Column::Email.eq(payload.email))
        .all(&state.db)
        .await;

    if let Ok(therapist_hits) = query_result {
        match therapist_hits.len() {
            0 => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"msg": "therapist not found"})),
                ));
            }
            1 => {
                let therapist = therapist_hits.first().unwrap();
                log::debug!("Single user found: {:?}", therapist.email);

                let verify_result = verify(payload.password, &therapist.password);
                log::debug!("Verification result: {:?}", verify_result);

                match verify_result {
                    Ok(true) => {
                        log::debug!("Successfully verified");
                        let jwt_result = jwt::generate(
                            &therapist.id.to_string(),
                            &therapist.email,
                            Some(therapist.role.clone()),
                        );

                        if let Ok(jwt) = jwt_result {
                            return Ok((StatusCode::OK, Json(TherapistSigninResponse { jwt })));
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
                    Json(serde_json::json!({"msg": "multiple therapists found"})),
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
