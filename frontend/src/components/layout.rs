#![allow(non_snake_case)]
use super::{footer::Footer, navbar::Navbar};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "site-root",
            // Skip-to-content for accessibility + crawlability
            a {
                href: "#main-content",
                class: "skip-link",
                "Skip to main content"
            }
            Navbar {}
            main {
                id: "main-content",
                role: "main",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
