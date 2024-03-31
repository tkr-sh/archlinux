use dioxus::prelude::*;
use super::logo::Logo;

#[component]
pub fn Home() -> Element {

    // Add event listener on scroll
    eval(
        r#"
        window.addEventListener("scroll", e => {
            const { scrollY } = window;
            const elem = document.getElementsByClassName("home-header")[0];

            if (scrollY > 50) {
                elem.classList.add("scroll");
            } else {
                elem.classList.remove("scroll");
            }
        });
        "#
    );

    rsx! {
        div {
            class: "Home",
            header {
                class: "home-header",
                div {
                    img {
                        src: "archlinux-gradient.svg"
                    }
                    b {"arch"}
                    span {class: "linux", "linux"}
                }
                div {
                    "a",
                    "b"
                }
            }
            Logo {}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
            br{}
            "uwu"
        }
    }
}
