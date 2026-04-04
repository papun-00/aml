#![allow(non_snake_case)]
use dioxus::prelude::*;

/// A reusable loading spinner component.
#[derive(Props, Clone, PartialEq)]
pub struct LoadingProps {
    #[props(default = "Loading...".to_string())]
    pub message: String,
}

#[component]
pub fn Loading(props: LoadingProps) -> Element {
    rsx! {
        div {
            class: "loading-container",
            role: "status",
            "aria-live": "polite",
            div { class: "loading-spinner", "aria-hidden": "true" }
            p { class: "loading-text", "{props.message}" }
        }
    }
}

/// A reusable error display component.
#[derive(Props, Clone, PartialEq)]
pub struct ErrorDisplayProps {
    pub message: String,
}

#[component]
pub fn ErrorDisplay(props: ErrorDisplayProps) -> Element {
    rsx! {
        div {
            class: "error-container",
            role: "alert",
            p { class: "error-message", "{props.message}" }
        }
    }
}
