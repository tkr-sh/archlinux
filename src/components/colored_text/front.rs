use dioxus::prelude::*;

#[component]
pub fn ColoredText(class: Option<&'static str>, href: Option<&'static str>, children: Element) -> Element {

    if href.is_some() {
        rsx! {
            a {
                href: href.unwrap(),
                class: r#"colored-text {class.unwrap_or("white")}"#,
                {children}
            }
        }
    } else {
        rsx! {
            span {
                class: r#"colored-text {class.unwrap_or("white")}"#,
                {children}
            }
        }
    }
}
