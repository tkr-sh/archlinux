use dioxus::prelude::*;

use crate::utils::switch_theme::switch_theme;

#[component]
pub fn Wiki(name: String) -> Element {
    switch_theme();
    switch_theme();
    let name_signal = use_signal(|| name.clone().replace("_", " "));
    // let mut body_signal = use_signal(|| String::new());
    let mut body_signal = use_resource(move || async move {
        get_wiki_page(name_signal())
            .await
            .replace("href=\"/title/", "href=\"/wiki/")
            .replace("/images/0/0b/Inaccurate.svg", "/assets/warn.svg")
    });

    let toc = use_resource(move || async move {
        get_toc_page(name_signal())
            .await
            .replace("href=\"/title/", "href=\"/wiki/")
            .replace("/images/0/0b/Inaccurate.svg", "/assets/warn.svg")
    });

    // Fedora
    // use_effect(move || {
    //     println!("{name_signal}");
    //     use_future(move || async move {
    //         println!("HEYYYYYYYYYYYYYYYYYYYYYYY");
    //         let body = get_wiki_page(name_signal()).await;
    //         println!("{body}");
    //         body_signal.set(body);
    //         println!("{body_signal}");
    //         println!("{body_signal}");
    //         println!("{body_signal}");
    //         println!("{body_signal}");
    //         println!("{body_signal}");
    //     });
    // });

    // use_resource(future)

    rsx! {
        div {
            class: "Wiki",
            nav {
                ul {
                    dangerous_inner_html: toc
                }
            }
            div {
                class: "outer-main",
                main {
                    h1 {"{name_signal}"}
                    match body_signal.read_unchecked().as_ref() {
                        Some(s) => rsx! { div {
                            id: "body",
                            dangerous_inner_html: s.to_string(),
                        }
                        },
                        None => None,
                    }
                }
            }
        }
    }
}


// #[server(GetWikiki)]
async fn get_wiki_page(name: String) -> String {
    println!("name : {name}");
    // let html = reqwest::get(&format!("https://wiki.archlinux.org/title/{name}")).await?.text().await?;
    let html = reqwest::get(&format!("http://localhost:8081/{name}")).await.unwrap().text().await.unwrap();
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    println!("{html}");
    let element = dom.get_element_by_id("bodyContent")
        .expect("Failed to find element")
        .get(parser)
        .unwrap();

    println!("element > {element:#?} <");

    element.inner_html(parser).to_string().replace("\t", "")
}

async fn get_toc_page(name: String) -> String {
    println!("name : {name}");
    // let html = reqwest::get(&format!("https://wiki.archlinux.org/title/{name}")).await?.text().await?;
    let html = reqwest::get(&format!("http://localhost:8081/{name}")).await.unwrap().text().await.unwrap();
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    println!("{html}");
    let element = dom.get_element_by_id("mw-panel-toc-list")
        .expect("Failed to find element")
        .get(parser)
        .unwrap();

    println!("element > {element:#?} <");

    element.inner_html(parser).to_string().replace("\t", "")
}
