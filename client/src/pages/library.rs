// src/pages/library.rs
use dioxus::prelude::*;

use crate::handlers::auth::AUTH_CONTEXT;

#[component]
pub fn LibraryPage() -> Element {
    if !AUTH_CONTEXT.read().is_authenticated {
        return rsx! {
            div { "Redirecting to login..." }
        };
    }

    rsx! {
        div {
            class: "min-h-screen bg-gray-100 p-4",
            div {
                class: "max-w-5xl mx-auto mt-6 bg-white rounded-lg p-6",
                h1 {
                    class: "text-2xl font-bold mb-4",
                    "Resources Library"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        class: "border border-gray-200 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Therapy Worksheets"
                        }
                        p {
                            "Browse and access worksheets for different therapy approaches."
                        }
                    }
                    div {
                        class: "border border-gray-200 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Assessment Tools"
                        }
                        p {
                            "Standard assessment tools and scoring guides."
                        }
                    }
                    div {
                        class: "border border-gray-200 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Educational Materials"
                        }
                        p {
                            "Handouts and resources for client education."
                        }
                    }
                    div {
                        class: "border border-gray-200 p-4 rounded-lg",
                        h2 {
                            class: "font-bold mb-2",
                            "Research Papers"
                        }
                        p {
                            "Recent research and evidence-based practices."
                        }
                    }
                }
            }
        }
    }
}
