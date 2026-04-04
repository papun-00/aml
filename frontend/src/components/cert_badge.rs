#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CertBadgeProps {
    pub name: String,
    pub icon: String,
}

#[component]
pub fn CertBadge(props: CertBadgeProps) -> Element {
    rsx! {
        div {
            class: "cert-item",
            role: "listitem",

            span {
                class: "cert-icon",
                "{props.icon}"
            }

            span {
                "{props.name}"
            }
        }
    }
}
