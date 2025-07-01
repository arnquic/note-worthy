// src/components/auth.rs
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub user: Option<User>,
    pub is_authenticated: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
        }
    }
}

pub static AUTH_CONTEXT: GlobalSignal<AuthState> = Global::new(|| AuthState::default());

pub fn login(username: String, password: String) -> bool {
    // This would be a real API call in a production app
    if username == "demo" && password == "password" {
        let user = User {
            id: "1".to_string(),
            username,
            display_name: "Jake Arnquist".to_string(),
            initials: "JA".to_string(),
        };
        *AUTH_CONTEXT.write() = AuthState {
            user: Some(user),
            is_authenticated: true,
        };
        true
    } else {
        false
    }
}

pub fn logout() {
    *AUTH_CONTEXT.write() = AuthState::default();
}
