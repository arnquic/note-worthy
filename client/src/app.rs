// src/app.rs
use dioxus::prelude::*;

use crate::pages::route::Route;

pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
