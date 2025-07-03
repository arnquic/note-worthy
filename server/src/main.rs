use sea_orm::ConnectOptions;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod controllers;
mod router;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub therapist_license_number: Option<String>,
    pub specializations: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let database_url = std::env::var("DATABASE_URL")?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    // Database connection setup
    let mut db_opts = ConnectOptions::new(database_url);
    db_opts
        .max_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .sqlx_logging(false);

    let db = sea_orm::Database::connect(db_opts).await?;

    // Create router
    let router = router::create_router(db);

    // Start server
    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;
    println!("Server running on http://localhost:3000");

    axum::serve(listener, router).await?;

    Ok(())
}
