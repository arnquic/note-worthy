// src/pages/home.rs
use dioxus::prelude::*;

use crate::handlers::auth::AUTH_CONTEXT;

#[component]
pub fn HomePage() -> Element {
    if !AUTH_CONTEXT.read().is_authenticated {
        return rsx! {
            div {
              "Redirecting to login..."
            }
        };
    }

    rsx! {
        div {
            class: "min-h-screen bg-gray-100 p-4",
            div {
                class: "max-w-5xl mx-auto mt-6 bg-white rounded-lg p-6",
                h1 {
                    class: "text-2xl font-bold mb-4",
                    "Dashboard"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    div {
                        class: "bg-blue-100 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Today's Appointments"
                        }
                        p {
                            "You have 3 appointments scheduled for today."
                        }
                    }
                    div {
                        class: "bg-green-100 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Client Updates"
                        }
                        p {
                            "2 clients have updates that need your attention."
                        }
                    }
                    div {
                        class: "bg-purple-100 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Tasks"
                        }
                        p {
                            "You have 5 pending tasks to complete."
                        }
                    }
                }
            }
        }
    }
}
