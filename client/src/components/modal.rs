use dioxus::prelude::*;

#[component]
pub fn Modal(
    on_close: EventHandler<()>,
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| on_close.call(()),
            
            div {
                class: "modal-content",
                onclick: move |e| e.stop_propagation(), // Prevent closing when clicking modal content
                
                div {
                    class: "modal-header",
                    h2 { "{title}" }
                    button {
                        class: "close-button",
                        onclick: move |_| on_close.call(()),
                        "×"
                    }
                }
                
                div {
                    class: "modal-body",
                    {children}
                }
                
                div {
                    class: "modal-footer",
                    button {
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}