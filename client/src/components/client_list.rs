// src/components/client_list.rs
use chrono::NaiveDateTime;
use dioxus::prelude::*;

use crate::{models::client::Client, pages::route::Route};

#[component]
pub fn ClientList(
    clients: Vec<Client>,
    #[props(optional)] on_client_select: Option<EventHandler<Client>>,
) -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "w-full",
            table {
                class: "w-full",
                thead {
                    tr {
                        class: "border-b border-gray-300 pb-2",
                        th {
                            class: "text-left pb-4 font-semibold",
                            "Name"
                        }
                        th {
                            class: "text-left pb-4 font-semibold",
                            "Status"
                        }
                        th {
                            class: "text-left pb-4 font-semibold",
                            "Next Appointment"
                        }
                        th {
                            class: "text-left pb-4 font-semibold",
                            "Last Appointment"
                        }
                    }
                }
                tbody {
                    {
                      clients.iter().map(|client| {
                      let row_class = if client.id == "1" {
                          "bg-blue-800 text-white"
                      } else {
                          "bg-white text-black"
                      };

                      let client_clone = client.clone();
                      rsx! {
                          tr {
                              key: "{client.id}",
                              class: "cursor-pointer {row_class}",
                              onclick: move |_| {
                                  if let Some(handler) = on_client_select {
                                      handler.call(client_clone.clone());
                                  }
                              },
                              td {
                                  class: "py-4",
                                  "{client.last_name}, {client.first_name}"
                              }
                              td {
                                  class: "py-4",
                                  "{client.status}"
                              }
                              td {
                                  class: "py-4",
                                  "{format_datetime(&client.next_appointment)}"
                              }
                              td {
                                  class: "py-4",
                                  "{format_datetime(&client.last_appointment)}"
                              }
                          }
                      }
                    })
                  }
                }
            }
            div {
                class: "flex justify-center mt-4",
                button {
                    class: "h-12 w-12 rounded-full bg-gray-800 flex items-center justify-center text-white",
                    onclick: move |_| {
                      navigator.push(Route::AddClient {});
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
}

fn format_datetime(dt: &Option<NaiveDateTime>) -> String {
    if let Some(dt) = dt {
        format!("{} - {}", dt.format("%d %B, %Y"), dt.format("%H:%M%P"))
    } else {
        "Not scheduled".to_string()
    }
}
