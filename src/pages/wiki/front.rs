use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::switch_theme::switch_theme;

#[component]
pub fn Wiki(name: String) -> Element {
    switch_theme();
    switch_theme();

    let name_signal = use_signal(|| name.clone().replace("_", " "));

    let ssr_body = use_server_future(move || async move {
        get_wiki_page(name_signal())
            .await
            .unwrap()
    });


    rsx! {
        div {
            class: "Wiki",
            if let Some(toc) = ssr_body.unwrap().read_unchecked().as_ref() {
                nav {
                    ul {
                        dangerous_inner_html: toc.toc.clone(),
                    }
                }
            }
            div {
                class: "outer-main",
                main {
                    h1 {"{name_signal}"}
                    match ssr_body.unwrap().read_unchecked().as_ref() {
                        Some(s) => rsx! { div {
                            id: "body",
                            dangerous_inner_html: s.body.clone(),
                        }
                        },
                        None => None,
                    }
                }
            }
        }
    }
}


// Data containig the body and the table of content
#[derive(Serialize, Deserialize)]
pub struct DataFromServer {
    body: String,
    toc: String,
}


#[server(GetWikiki)]
async fn get_wiki_page(name: String) -> Result<DataFromServer, ServerFnError> {
    // In a first place get the HTML
    let html = reqwest::get(&format!("http://localhost:8081/{name}")).await.unwrap().text().await.unwrap();

    // Initalize the dom with tl
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();

    // Create a parsedr
    let parser = dom.parser();

    // Get the raw html of the body
    let body = dom.get_element_by_id("bodyContent")
        .expect("Failed to find element")
        .get(parser)
        .unwrap()
        .inner_html(parser)
        .to_string()
        .replace("\t", "")
        .replace("href=\"/title/", "href=\"/wiki/")
        .replace("/images/0/0b/Inaccurate.svg", "/assets/warn.svg");

    // Get the raw HTML of the navbar
    let toc = dom.get_element_by_id("mw-panel-toc-list")
        .expect("Failed to find element")
        .get(parser)
        .unwrap()
        .inner_html(parser)
        .to_string()
        .replace("\t", "");


    Ok(DataFromServer { toc, body })
}
