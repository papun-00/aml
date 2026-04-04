#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use super::{navbar::Navbar, footer::Footer};
use crate::Route;

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
