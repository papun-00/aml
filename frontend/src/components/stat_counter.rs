#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatCounterProps {
    pub label: String,
    pub value: String,
    pub description: String,
}

#[component]
pub fn StatCounter(props: StatCounterProps) -> Element {
    rsx! {
        div {
            class: "stat-item",

            dt {
                class: "stat-label",
                "{props.label}"
            }

            dd {
                class: "stat-value",
                "{props.value}"
            }

            dd {
                class: "stat-desc",
                "{props.description}"
            }
        }
    }
}
