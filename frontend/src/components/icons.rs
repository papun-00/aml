#![allow(non_snake_case)]
//! IBM Carbon Design System SVG icons as Dioxus components.
//! All icons use explicit width/height for reliable WASM rendering.
use dioxus::prelude::*;

/// Carbon: ChevronRight 16px
#[component]
pub fn ChevronRight(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M22 16L12 26l-1.414-1.414L19.172 16 10.586 7.414 12 6z" }
    }}
}

/// Carbon: Checkmark 16px
#[component]
pub fn Check(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M14 21.414L9 16.413 10.413 15 14 18.586 21.585 11 23 12.414z" }
    }}
}

/// Carbon: Email 16px
#[component]
pub fn Mail(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M28 6H4a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h24a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2zm-2.2 2L16 14.78 6.2 8zM4 24V8.91l11.43 7.91a1 1 0 0 0 1.14 0L28 8.91V24z" }
    }}
}

/// Carbon: Earth 16px
#[component]
pub fn Earth(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M16 2A14 14 0 1 0 30 16 14 14 0 0 0 16 2zm12 13h-3a23 23 0 0 0-1.8-8.2A12 12 0 0 1 28 15zM16 28c-1.5 0-3.4-3.4-3.9-11h7.8c-.5 7.6-2.4 11-3.9 11zm-3.9-13C12.6 7.4 14.5 4 16 4s3.4 3.4 3.9 11zM8.8 6.8A23 23 0 0 0 7 15H4a12 12 0 0 1 4.8-8.2zM4 17h3a23 23 0 0 0 1.8 8.2A12 12 0 0 1 4 17zm19.2 8.2A23 23 0 0 0 25 17h3a12 12 0 0 1-4.8 8.2z" }
    }}
}

/// Carbon: StarFilled 16px
#[component]
pub fn StarFilled(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M16 2l4.2 8.6 9.4 1.4-6.8 6.6 1.6 9.4L16 23.8 7.6 28l1.6-9.4L2.4 12l9.4-1.4z" }
    }}
}

/// Carbon: ArrowRight 16px
#[component]
pub fn ArrowRight(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M18 6l-1.4 1.4L23.2 14H4v2h19.2l-6.6 6.6L18 24l10-10z" }
    }}
}

/// Carbon: Scale 16px (for quantity/weight)
#[component]
pub fn Scale(#[props(default = 16)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M30 14h-2V8h-6V6h8zM10 26H2V18h2v6h6z" }
        path { d: "M17 24H15V21.22L7.38 13.59 8.79 12.18 16 19.39 23.21 12.18 24.62 13.59 17 21.22z" }
    }}
}

/// Carbon: Light (sun) 20px — for theme toggle
#[component]
pub fn SunIcon(#[props(default = 20)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M16 12.005a4 4 0 1 1-4 4 4.005 4.005 0 0 1 4-4m0-2a6 6 0 1 0 6 6 6 6 0 0 0-6-6zM5.394 6.813L6.81 5.399l3.505 3.506L8.9 10.32zM2 15.005h5v2H2zM5.394 25.197l3.504-3.505 1.414 1.415-3.504 3.504zM15 25.005h2v5h-2zM21.687 21.692l1.414-1.415 3.505 3.505-1.414 1.415zM25 15.005h5v2h-5zM21.687 10.318L25.19 6.813l1.415 1.414-3.505 3.506zM15 2.005h2v5h-2z" }
    }}
}

/// Carbon: Asleep (moon) 20px — for theme toggle
#[component]
pub fn MoonIcon(#[props(default = 20)] size: u32) -> Element {
    rsx! { svg {
        width: "{size}",
        height: "{size}",
        view_box: "0 0 32 32",
        fill: "currentColor",
        "aria-hidden": "true",
        path { d: "M13.503 5.414a15.076 15.076 0 0 0 11.593 18.194 11.113 11.113 0 0 1-7.975 3.39c-.138 0-.278 0-.418-.005a11.1 11.1 0 0 1-3.2-21.58M14.98 3a1.002 1.002 0 0 0-.175.016 13.096 13.096 0 0 0 1.825 25.981c.164.006.328.006.49.006a13.072 13.072 0 0 0 10.703-5.555 1.01 1.01 0 0 0-.783-1.565A13.08 13.08 0 0 1 15.89 3.876 1.008 1.008 0 0 0 14.98 3z" }
    }}
}
