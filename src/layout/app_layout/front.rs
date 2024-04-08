use dioxus::prelude::*;
use crate::layout::header::front::Header;

#[component]
pub fn AppLayout() -> Element {

    rsx! {
        Header {}
        div {
            class: "AppLayout",
        }
    }
}
