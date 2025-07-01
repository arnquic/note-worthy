// src/pages/not_found.rs
use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center justify-center bg-gray-100",
            h1 {
                class: "text-4xl font-bold text-gray-800 mb-4",
                "404 - Page Not Found"
            }
            p {
                class: "text-gray-600 mb-8",
                "The page you are looking for doesn't exist or has been moved."
            }
        }
    }
}
