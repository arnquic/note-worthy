// src/components/nav_bar.rs
use dioxus::prelude::*;

use crate::handlers::auth::{AUTH_CONTEXT, logout};
use crate::pages::route::Route;

#[component]
pub fn NavBar() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "w-full bg-blue-800 rounded-full flex items-center justify-between px-4 py-2",
            div {
                class: "flex items-center space-x-4",
                NavButton {
                    label: "Home".to_string(),
                    my_route: Route::HomePage {  }
                },
                NavButton {
                    label: "Clients".to_string(),
                    my_route: Route::ClientsPage {  }
                },
                NavButton {
                    label: "Library".to_string(),
                    my_route: Route::LibraryPage {  }
                },
                NavButton {
                    label: "Organization".to_string(),
                    my_route: Route::OrganizationPage {  }
                }
            },
            div {
                class: "flex items-center",
                if let Some(user) = AUTH_CONTEXT.read().user.clone() {
                    div {
                        class: "ml-4 h-10 w-10 rounded-full bg-white flex items-center justify-center text-blue-800 font-bold",
                        "{user.initials}"
                    }
                } else {
                    button {
                        class: "ml-4 bg-white text-blue-800 px-4 py-2 rounded-full font-bold",
                        onclick: move |_| {
                            navigator.push(Route::LoginPage {  });
                        },
                        "Login"
                    }
                }
            }
        }
        Outlet::<Route>{}
    }
}

#[component]
fn NavButton(label: String, my_route: Route) -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();
    let active = matches!(route, my_route);

    let active_class = if active {
        "bg-black text-white"
    } else {
        "bg-transparent text-white"
    };

    rsx! {
        button {
            class: "px-6 py-2 rounded-full text-center font-medium transition-colors {active_class}",
            onclick: move |_| {
              navigator.push(my_route.clone());
            },
            "{label}"
        }
    }
}
