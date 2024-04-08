use std::time::Duration;

// use dioxus_fullstack::prelude::*;
use dioxus::prelude::*;
use regex::Regex;

use crate::components::colored_text::front::ColoredText;


#[component]
pub fn Packages() -> Element {
    let packages = [
        ("assets/librewolf.svg", "Librewolf", "Community-maintained fork of Firefox, focused on privacy, security and freedom."),
        ("assets/spotify.svg", "Spotify", "A proprietary music streaming service"),
        ("assets/proton.svg", "Proton", "Proton official desktop application for Proton Mail and Proton Calendar"),
        ("assets/vscodium.svg", "VSCodium", "Binary releases of VS Code without MS branding/telemetry/licensing."),
        ("assets/onlyoffice.svg", "OnlyOffice", "An office suite that combines text, spreadsheet and presentation editors"),
        ("assets/bun.svg", "Bun", "All-in-one JavaScript runtime with bundler, transpiler, test runner, and package manager."),
    ];

    let mut number_of_packages = use_signal(|| 0);
    let mut gentoo_packages = use_signal(|| 1);
    let mut fedora_packages = use_signal(|| 1);

    // Gentoo
    use_future(move || async move {
        let packages = get_gentoo_packages().await.unwrap_or(1);
        gentoo_packages.set(packages);
    });

    // Fedora
    use_future(move || async move {
        let packages = get_fedora_packages().await.unwrap_or(1);
        fedora_packages.set(packages);
    });

    use_future(move || async move {
        let dest_number_of_packages = get_arch_packages().await.unwrap_or(0);

        loop {
            async_std::task::sleep( Duration::from_millis( 20 ) ).await;

            let read = number_of_packages();
            number_of_packages += (dest_number_of_packages - read) / 15 + 7;

            if dest_number_of_packages - read < 100 {
                number_of_packages += 1;
            } else if dest_number_of_packages - read < 1000 {
                number_of_packages += (dest_number_of_packages - read) / 100;
            }

            if number_of_packages() >= dest_number_of_packages {
                number_of_packages.set(dest_number_of_packages);
                break;
            }
        }
    });


    rsx! {
        section {
            class: "package",
            div { class: "gradient blue" }
            div { class: "gradient purple" }
            div { class: "gradient dark-blue" }
            div { class: "gradient red" }
            h1 {
                if number_of_packages.read().to_owned() == 0 {
                    ColoredText {
                        "A lot of packages"
                    }
                } else {
                    ColoredText {
                        class: "blue",
                        "{number_of_packages}"
                    }
                    ColoredText {
                        class: "white",
                        " packages"
                    }
                }
            }
            div {
                class: "others",
                h2 {
                    ColoredText {
                        class: "purple",
                        "{ number_of_packages() as f64 / gentoo_packages() as f64 :.2}x"
                    }
                    ColoredText { " more than " }
                    ColoredText {
                        href: "https://gentoo.org",
                        class: "purple",
                        "Gentoo"
                    }
                }
                h2 {
                    ColoredText {
                        class: "red",
                        "{ number_of_packages() as f64 / 66243.0:.2}x"
                    }
                    ColoredText { " more than " }
                    ColoredText {
                        href: "https://debian.org",
                        class: "red",
                        "Debian"
                    }
                }
                h2 {
                    ColoredText {
                        class: "dark-blue",
                        "{number_of_packages() as f64 / fedora_packages() as f64:.2}x"
                    }
                    ColoredText { " more than " }
                    ColoredText {
                        class: "dark-blue",
                        href: "https://fedora.org",
                        "Fedora"
                    }
                }
            }
            table {
                thead {
                    tr {
                        th { "Logo" }
                        th { "Name" }
                        th { "Description" }
                    }
                }
                tbody {
                    for package in packages {
                        tr {
                            td { img { src: package.0} }
                            td { b { "{package.1}" }}
                            td { "{package.2}" }
                        }
                    }
                }
            }
            span { "None of theses packages were available in either Debian or Fedora repository at April 1st 2024" }
        }
    }
}



#[server]
async fn get_arch_packages() -> Result<usize, ServerFnError> {
    let re = Regex::new(r"(?<number>\d+) (matching )?packages found").unwrap();
    // AUR
    let aur_html = reqwest::get("https://aur.archlinux.org/packages/").await?.text().await?;
    let aur_packages = re
        .captures(&aur_html)
        .map(|mtch| mtch["number"].parse().unwrap())
        .unwrap_or(0);
    // Arch
    let arch_html = reqwest::get("https://archlinux.org/packages/").await?.text().await?;
    let arch_packages = re.captures(&arch_html).map(|mtch| mtch["number"].parse().unwrap()).unwrap_or(0);

    Ok(aur_packages + arch_packages)
}


#[server(GetFedora)]
async fn get_fedora_packages() -> Result<usize, ServerFnError> {
    let re = Regex::new(r#"(?<number>\d+) packages\)\."#).unwrap();

    // Parse
    let html = reqwest::get("https://packages.fedoraproject.org/").await?.text().await?;
    let packages = re
        .captures(&html)
        .map(|mtch| mtch["number"].parse().unwrap())
        .unwrap_or(0);

    Ok(packages)
}

#[server(GetGentoo)]
async fn get_gentoo_packages() -> Result<usize, ServerFnError> {
    let re = Regex::new(r#"class="text-primary">(?<thousand>\d+),(?<unit>\d+)</"#).unwrap();

    // Parse
    let html = reqwest::get("https://packages.gentoo.org/").await?.text().await?;
    let packages = re
        .captures(&html)
        .map(|mtch| mtch["thousand"].parse::<usize>().unwrap() * 1000 + mtch["unit"].parse::<usize>().unwrap())
        .unwrap_or(0);

    Ok(packages)
}
