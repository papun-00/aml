#![allow(non_snake_case)]
use super::theme_toggle::ThemeToggle;
use crate::utils::asset_url;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);
    let logo_src = asset_url("/assets/alashore-logo.png");

    rsx! {
        header {
            class: "site-header",
            role: "banner",
            // Semantic: header contains nav — good for AI/screen readers
            nav {
                class: "nav-container",
                role: "navigation",
                "aria-label": "Main navigation",
                itemscope: "true",
                itemtype: "https://schema.org/SiteNavigationElement",

                // Logo — schema.org Organization link
                Link {
                    to: Route::HomePage {},
                    class: "nav-logo",
                    "aria-label": "Alashore Marine Exports – Home",
                    itemprop: "url",
                    img {
                        src: "{logo_src}",
                        alt: "Alashore Marine Export Pvt. Ltd. Logo",
                        style: "height:2.25rem;width:auto;object-fit:contain;",
                    }
                }

                // Desktop nav links — itemprop for schema
                ul {
                    class: "nav-links",
                    role: "list",
                    li { itemprop: "name",
                        Link { to: Route::HomePage {}, class: "nav-link", itemprop: "url", "Home" }
                    }
                    li { itemprop: "name",
                        Link { to: Route::AboutPage {}, class: "nav-link", itemprop: "url", "About" }
                    }
                    li { itemprop: "name",
                        Link { to: Route::ProductsPage {}, class: "nav-link", itemprop: "url", "Products" }
                    }
                    li { itemprop: "name",
                        Link { to: Route::CertificationsPage {}, class: "nav-link", itemprop: "url", "Certifications" }
                    }
                    li { itemprop: "name",
                        Link { to: Route::SustainabilityPage {}, class: "nav-link", itemprop: "url", "Sustainability" }
                    }
                    li { itemprop: "name",
                        Link { to: Route::ContactPage {}, class: "nav-link", itemprop: "url", "Contact" }
                    }
                }

                // Theme toggle
                ThemeToggle {}

                // CTA
                Link {
                    to: Route::InquiryPage {},
                    class: "nav-cta",
                    "aria-label": "Request a seafood price quote",
                    "Request a Quote"
                }

                // Mobile hamburger
                button {
                    class: "nav-hamburger",
                    "aria-label": "Toggle navigation menu",
                    "aria-expanded": "{menu_open}",
                    "aria-controls": "mobile-menu",
                    onclick: move |_| menu_open.set(!menu_open()),
                    span { class: if menu_open() { "ham-line open" } else { "ham-line" } }
                    span { class: if menu_open() { "ham-line open" } else { "ham-line" } }
                    span { class: if menu_open() { "ham-line open" } else { "ham-line" } }
                }
            }

            // Mobile menu
            div {
                id: "mobile-menu",
                class: if menu_open() { "mobile-nav open" } else { "mobile-nav" },
                "aria-hidden": "{!menu_open()}",
                ul { role: "list",
                    li { Link { to: Route::HomePage {},           onclick: move |_| menu_open.set(false), "Home" } }
                    li { Link { to: Route::AboutPage {},          onclick: move |_| menu_open.set(false), "About" } }
                    li { Link { to: Route::ProductsPage {},       onclick: move |_| menu_open.set(false), "Products" } }
                    li { Link { to: Route::CertificationsPage {}, onclick: move |_| menu_open.set(false), "Certifications" } }
                    li { Link { to: Route::SustainabilityPage {}, onclick: move |_| menu_open.set(false), "Sustainability" } }
                    li { Link { to: Route::ContactPage {},        onclick: move |_| menu_open.set(false), "Contact" } }
                    li { Link { to: Route::InquiryPage {},        class: "mobile-cta", onclick: move |_| menu_open.set(false), "Request a Quote" } }
                }
            }
        }
    }
}
