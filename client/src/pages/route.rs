use dioxus::prelude::*;

use crate::components::nav_bar::NavBar;
use crate::pages::{
    add_client::AddClient, clients::ClientsPage, home::HomePage, library::LibraryPage,
    login::LoginPage, not_found::NotFoundPage, organization::OrganizationPage,
};

#[component]
pub fn ExamplePage() -> Element {
    rsx! {
      div {
        "This is an example page"
      }
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
      #[route("/")]
      HomePage {},
      #[nest("/clients")]
        #[route("/")]
        ClientsPage {},
        #[route("/add")]
        AddClient {},
      #[end_nest]
      #[route("/library")]
      LibraryPage {},
      #[route("/organization")]
      OrganizationPage {},
      #[route("/login")]
      LoginPage {},
    #[end_layout]
    #[route("/:..segments")]
    NotFoundPage { segments: Vec<String> },
}
