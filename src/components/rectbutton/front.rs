use dioxus::prelude::*;

#[component]
pub fn Rectbutton(r#type: &'static str, text: &'static str) -> Element {

    rsx! {
        button {
            class: "Rectbutton {r#type}",
            span { "{text}" }
        }
    }
}
