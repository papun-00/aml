#![allow(non_snake_case)]
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        // No index for 404
        document::Meta { name: "robots", content: "noindex, follow" }
        document::Title { "Page Not Found | Alashore Marine Exports" }

        section { class: "not-found-page",
            div { class: "not-found-content",
                div { class: "not-found-code", "404" }
                h1 { class: "not-found-title", "Page Not Found" }
                p { class: "not-found-desc",
                    "The page "
                    code { "/{path}" }
                    " doesn't exist."
                }
                p { class: "not-found-hint",
                    "You may be looking for one of these:"
                }
                nav { class: "not-found-links", "aria-label": "Suggested pages",
                    Link { to: Route::HomePage {},           class: "nf-link", "Home" }
                    Link { to: Route::ProductsPage {},       class: "nf-link", "Products" }
                    Link { to: Route::AboutPage {},          class: "nf-link", "About" }
                    Link { to: Route::CertificationsPage {}, class: "nf-link", "Certifications" }
                    Link { to: Route::ContactPage {},        class: "nf-link", "Contact" }
                    Link { to: Route::InquiryPage {},        class: "nf-link nf-link-cta", "Request a Quote" }
                }
            }
            div { class: "not-found-visual", "aria-hidden": "true",
                div { class: "nf-wave" }
            }
        }
    }
}
