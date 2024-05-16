use std::collections::HashMap;

use dioxus::prelude::*;
use tl::NodeHandle;
use itertools::Itertools;


use crate::{components::alphabetical_column::front::AlphabeticalColumn, shared::structs::CategoryWord, utils::switch_theme::switch_theme};

#[component]
pub fn WikiCategory(name: String) -> Element {
    switch_theme();
    switch_theme();
    let signal_name = use_signal(||name);

    let category = use_server_future(move || async move {
        get_category(signal_name())
            .await
    });

    let hashmap_letters = use_memo(move || {
        category
            .map(|cat| {
                if let Ok(ok_cat) = cat.unwrap() {
                    Some(
                        ok_cat
                            .into_iter()
                            .group_by(|item| item.name.chars().next().unwrap())
                            .into_iter()
                            .map(|(key, group)| (key, group.collect()))
                            .collect::<HashMap<char, Vec<CategoryWord>>>()
                    )
                } else {
                    None
                }
            }).flatten()
    });

    rsx! {
        div {
            class: "WikiCategory",

            main {
                h1 {
                    i { "Category: "}
                    "{signal_name}"
                }
                if let Some(hm) = hashmap_letters() {
                    for (letter, words) in hm.into_iter().sorted_by_key(|(l,_)|*l) {
                        AlphabeticalColumn {
                            letter,
                            words,
                        }
                    }
                }
            }
        }
    }
}

#[server(GetCategory)]
async fn get_category(name: String) -> Result<Vec<CategoryWord>, ServerFnError> {
    // In a first place get the HTML
    let html = reqwest::get(&format!("http://localhost:8081/Category:{name}")).await?.text().await?;

    // Initalize the dom with tl
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();

    // Create a parsedr
    let parser = dom.parser();

    // Get the raw html of the body
    let mut groups = dom.get_elements_by_class_name("mw-category-columns");
    // All of that, just to get anchors. This lib is bloated
    let groups_anchor_node = groups
        .next()
        .unwrap()
        .clone()
        .get(parser)
        .unwrap();
    let groups_anchor_tag = groups_anchor_node.as_tag();
    let groups_anchor = groups_anchor_tag.unwrap().query_selector(parser, "a");

    let mut words = Vec::<CategoryWord>::new();

    if let Some(groups) = groups_anchor {
        for group in groups {
            println!("{group:#?}");
            let node = group.get(parser).unwrap().clone();
            let tag = node.as_tag().unwrap().to_owned();
            let links_to = tag.attributes().get("href").unwrap().map(|b| b.as_utf8_str().to_string()).unwrap();
            let text = tag.inner_text(parser);
            words.push(CategoryWord { name: text.to_string(), links_to })
        }
    }


    Ok(words)
}
