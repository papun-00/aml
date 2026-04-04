#![allow(non_snake_case)]
// SVG icons as Dioxus components
use dioxus::prelude::*;

#[component]
pub fn ChevronRight(#[props(default = "w-4 h-4")] class: &'static str) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 16 16", fill: "currentColor",
        path { d: "M6 3l5 5-5 5" }
    }}
}

#[component]
pub fn Check(#[props(default = "w-4 h-4")] class: &'static str) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 16 16", fill: "currentColor",
        path { d: "M13.5 2.5L6 10 2.5 6.5 1 8l5 5 9-9z" }
    }}
}

#[component]
pub fn Mail(#[props(default = "w-4 h-4")] class: &'static str) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { stroke_linecap: "round", stroke_linejoin: "round",
            d: "M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75"
        }
    }}
}
