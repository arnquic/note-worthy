use chrono::{Duration, Utc};
use entity::sea_orm_active_enums::Role;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // Subject (user ID)
    exp: i64,    // Expiration time
    iat: i64,    // Issued at time
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<Role>,
}

pub fn generate(
    id: &str,
    email: &str,
    role: Option<Role>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let now = Utc::now();

    let claims = Claims {
        sub: id.to_string(),
        exp: now
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .timestamp(),
        iat: now.timestamp(),
        email: email.to_string(),
        role,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
