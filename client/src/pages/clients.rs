// src/pages/clients.rs
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use log::logger;

use crate::components::{client_list::ClientList, modal::Modal};
use crate::handlers::auth::AUTH_CONTEXT;
use crate::models::client::{Client, ClientStatus};
use crate::pages::add_client::AddClient;

#[component]
pub fn ClientsPage() -> Element {
    // Mock client data
    let clients = vec![
        Client {
            id: "1".to_string(),
            first_name: "Jake".to_string(),
            last_name: "Arnquist".to_string(),
            status: ClientStatus::Canceled,
            next_appointment: NaiveDateTime::parse_from_str(
                "2025-03-22 15:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
            last_appointment: NaiveDateTime::parse_from_str(
                "2025-03-15 15:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
        },
        Client {
            id: "2".to_string(),
            first_name: "Elizabeth".to_string(),
            last_name: "Hackney".to_string(),
            status: ClientStatus::InTherapy,
            next_appointment: NaiveDateTime::parse_from_str(
                "2025-03-22 15:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
            last_appointment: NaiveDateTime::parse_from_str(
                "2025-03-15 15:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
        },
    ];

    // If not authenticated, redirect to login
    if !AUTH_CONTEXT.read().is_authenticated {
        return rsx! {
            div { "Redirecting to login..." }
        };
    }

    let mut show_modal = use_signal(|| false);

    rsx! {
        div {
            class: "min-h-screen bg-gray-100 p-4",
            div {
                class: "max-w-5xl mx-auto mt-6 bg-white rounded-lg p-6",
                ClientList {
                    clients: clients.clone(),
                    on_client_select: move |client: Client| {
                        log::info!("Selected client: {}", client.id);
                    }
                }
                div {
                  class: "flex justify-center mt-4",
                  button {
                      class: "h-12 w-12 rounded-full bg-gray-800 flex items-center justify-center text-white",
                      onclick: move |_| {
                        show_modal.set(true);
                        
                      },
                      svg {
                          xmlns: "http://www.w3.org/2000/svg",
                          class: "h-6 w-6",
                          fill: "none",
                          view_box: "0 0 24 24",
                          stroke: "currentColor",
                          path {
                              stroke_linecap: "round",
                              stroke_linejoin: "round",
                              stroke_width: "2",
                              d: "M12 4v16m8-8H4"
                          }
                      }
                  }
              }
            }
        }

        if show_modal() {
          Modal {
            on_close: move |_| {show_modal.set(false)},
            title: "Add Client",
            children: rsx! {
              AddClient{}
            }
          }
        }
    }
}
