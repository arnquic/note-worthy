use crate::controllers::{client, therapist};
use axum::{Router, routing::post};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub fn create_router(db: DatabaseConnection) -> Router {
    let state = Arc::new(AppState { db });

    Router::new()
        .route("/client", post(client::create))
        .route("/client/signin", post(client::signin))
        .route("/therapist", post(therapist::create))
        .route("/therapist/signin", post(therapist::signin))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
