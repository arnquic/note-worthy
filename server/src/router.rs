use crate::controllers::client;
use axum::{
    Router,
    routing::{get, post},
};
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
        .layer(CorsLayer::permissive())
        .with_state(state)
}
