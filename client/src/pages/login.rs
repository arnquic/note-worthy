// src/pages/login.rs
use dioxus::prelude::*;

use crate::handlers::auth::{AUTH_CONTEXT, login};
use crate::pages::route::Route;

#[component]
pub fn LoginPage() -> Element {
    let navigator = navigator();
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    // If already authenticated, redirect to home
    if AUTH_CONTEXT.read().is_authenticated {
        return rsx! {
            div { "Redirecting to home..." }
        };
    }

    let handle_login = move |_| {
        if username.read().is_empty() || password.read().is_empty() {
            error.set(Some("Username and password are required".to_string()));
            return;
        }

        // In a real app, this would be an async API call
        let success = login(username.read().to_string(), password.read().to_string());

        if success {
            error.set(None);
            navigator.push(Route::HomePage {});
        } else {
            error.set(Some("Invalid username or password".to_string()));
        }
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gray-100",
            div {
                class: "max-w-md w-full bg-white rounded-lg shadow-lg p-8",
                div {
                    class: "text-center mb-8",
                    h1 {
                        class: "text-3xl font-bold text-blue-800",
                        "Therapy Client Manager"
                    }
                    p {
                        class: "text-gray-600",
                        "Sign in to your account"
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    div {
                        class: "mb-4 p-3 bg-red-100 text-red-800 rounded",
                        "{err}"
                    }
                }

                div {
                    form {
                        class: "space-y-4",
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Username"
                            }
                            input {
                                class: "w-full p-2 border border-gray-300 rounded",
                                r#type: "text",
                                value: "{username}",
                                oninput: move |evt| username.set(evt.value())
                            }
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Password"
                            }
                            input {
                                class: "w-full p-2 border border-gray-300 rounded",
                                r#type: "password",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value())
                            }
                        input {
                            class: "w-full bg-blue-800 text-white py-2 rounded font-medium hover:bg-blue-900",
                            r#type: "submit",
                            value: "Sign In",
                            onclick: handle_login,
                        }
                    }
                    div {
                        class: "text-center mt-4 text-sm text-gray-600",
                        p {
                            "Demo credentials: demo / password"
                        }
                    }
                }
            }
        }
    }
}
