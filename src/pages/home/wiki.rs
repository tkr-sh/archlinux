
use std::time::Duration;

use dioxus::prelude::*;
use regex::Regex;

use crate::components::colored_text::front::ColoredText;


#[component]
pub fn Wiki() -> Element {
    rsx! {
        div {
            class: "wiki",
            h1 {
                ColoredText { "Meet the " }
                ColoredText { class: "blue", "Wiki" }
            }
            p {
                "Discover the "
                b { "ultimate" }
                " resource hub for mastering Arch Linux - your go-to wiki for "
                b { "in-depth guides" }
                ", "
                b{ "troubleshooting tips" }
                ", and "
                b { "community-driven knowledge"}
                "."
            }
            a {
                class: "outer-img",
                href: "https://wiki.archlinux.org/"
            }
        }
    }
}
