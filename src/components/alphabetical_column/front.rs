use dioxus::prelude::*;

use crate::shared::structs::CategoryWord;

#[component]
pub fn AlphabeticalColumn(letter: String, words: Vec<CategoryWord>) -> Element {
    rsx! {
        div {
            class: "AlphabeticalColumn",
            h2 {
                span { class: "letter", "{letter}" }
                " - {words.len()}"
            }
            div { class: "spacer" }
            ul {
                for word in words {
                    li {
                        a {
                            href: word.links_to,
                            "{word.name}"
                        }
                    }
                }
            }
        }
    }
}
