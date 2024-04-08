use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::{IoMail, IoMailOutline, IoPeopleOutline, IoTerminal, IoTerminalOutline}, Icon};

use crate::components::colored_text::front::ColoredText;

#[component]
pub fn Community() -> Element {
    rsx! {
        section {
            class: "community",
            h1 {
                ColoredText { class: "white", "Arch"}
                ColoredText { class: "black", " is a "}
                ColoredText { class: "white", "community"}
            }
            p {
r#"Our large community is diverse and supportive, and we take pride in the wide range of skills and uses for Arch that emerge from it.
Arch is built by people like you, and this is reflected in the AUR, a community-operated package repository that grows in size and quality every day because of you."#
            }

            div {
                class: "links",
                a {
                    href: "https://wiki.archlinux.org/title/IRC_channels",
                    // Icon {
                    //     width: 50,
                    //     height: 50,
                    //     fill: "white",
                    //     icon: IoTerminal,
                    // }
                    "IRC"
                }
                a {
                    href: "https://lists.archlinux.org/mailman3/lists/",
                    // Icon {
                    //     width: 50,
                    //     height: 50,
                    //     fill: "white",
                    //     icon: IoMail,
                    // }
                    "Mail list"
                }
                a {
                    href: "https://bbs.archlinux.org/",
                    // Icon {
                    //     width: 50,
                    //     height: 50,
                    //     fill: "white",
                    //     icon: IoPeopleOutline,
                    // }
                    "Forum"
                }
            }
        }
    }
}
