// src/main.rs
mod app;
mod components;
mod handlers;
mod models;
mod pages;
mod services;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(App);
}
