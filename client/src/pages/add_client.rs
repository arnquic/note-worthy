use dioxus::prelude::*;

#[component]
pub fn AddClient() -> Element {
    rsx! {
      div {
        class: "",
        h1 {
          "Enter Client Info"
        }
        form {
          label {
            "First Name"
          }
          input {
            r#type: "text",
            value: "Jane"
          }
          label {
            "Last Name"
          }
          input {
            r#type: "text",
            value: "Doe"
          }
        }
      }
    }
}
