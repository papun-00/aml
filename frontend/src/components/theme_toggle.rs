#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::document::eval;
use crate::components::icons::{SunIcon, MoonIcon};

/// Theme toggle using IBM Carbon Design System icons (Light/Asleep).
#[component]
pub fn ThemeToggle() -> Element {
    let mut is_dark = use_signal(|| false);

    rsx! {
        button {
            class: "theme-toggle",
            "aria-label": if is_dark() { "Switch to light theme" } else { "Switch to dark theme" },
            title: if is_dark() { "Switch to light theme" } else { "Switch to dark theme" },
            onclick: move |_| {
                let new_dark = !is_dark();
                is_dark.set(new_dark);
                eval(&format!(
                    r#"
                    if ({dark}) {{
                        document.documentElement.classList.add('dark');
                        localStorage.setItem('theme', 'dark');
                    }} else {{
                        document.documentElement.classList.remove('dark');
                        localStorage.setItem('theme', 'light');
                    }}
                    "#,
                    dark = if new_dark { "true" } else { "false" }
                ));
            },
            span {
                class: "theme-toggle-icon",
                "aria-hidden": "true",
                if is_dark() {
                    SunIcon { size: 20 }
                } else {
                    MoonIcon { size: 20 }
                }
            }
        }
    }
}
