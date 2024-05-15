#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::layout::app_layout::front::AppLayout;
// use dioxus_fullstack::prelude::*;
use log::LevelFilter;

mod pages;
mod components;
mod utils;
mod shared;
mod layout;
use pages::home::front::Home;
use pages::wiki::front::Wiki;
use pages::wiki::category::front::WikiCategory;


#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    // TODO
    #[route("/wiki/:name")]
    Wiki { name: String },
    #[route("/wiki/category/:name")]
    WikiCategory { name: String },
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
