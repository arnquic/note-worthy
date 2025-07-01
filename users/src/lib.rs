// src/models/client.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientStatus {
    InTherapy,
    OnHold,
    Completed,
    Canceled,
}

impl Display for ClientStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientStatus::InTherapy => write!(f, "In Therapy"),
            ClientStatus::OnHold => write!(f, "On Hold"),
            ClientStatus::Completed => write!(f, "Completed"),
            ClientStatus::Canceled => write!(f, "Canceled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Client {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub status: ClientStatus,
    pub next_appointment: Option<NaiveDateTime>,
    pub last_appointment: Option<NaiveDateTime>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
