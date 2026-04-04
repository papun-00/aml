#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct InquiryCtaProps {
    pub title: String,
    pub description: String,
    #[props(default = "Request a Quote".to_string())]
    pub primary_label: String,
    #[props(default = "Talk to Our Team".to_string())]
    pub secondary_label: String,
    #[props(default)]
    pub primary_route: Option<String>,
    #[props(default)]
    pub secondary_route: Option<String>,
}

#[component]
pub fn InquiryCta(props: InquiryCtaProps) -> Element {
    rsx! {
        section {
            class: "cta-section",

            div {
                class: "cta-inner",

                h2 {
                    class: "cta-title",
                    "{props.title}"
                }

                p {
                    class: "cta-desc",
                    "{props.description}"
                }

                div {
                    class: "cta-actions",

                    Link {
                        to: Route::InquiryPage {},
                        class: "btn btn-primary btn-lg",
                        "{props.primary_label}"
                    }

                    Link {
                        to: Route::ContactPage {},
                        class: "btn btn-outline btn-lg",
                        "{props.secondary_label}"
                    }
                }
            }
        }
    }
}
