#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CertBadgeProps {
    pub id: String,
    pub name: String,
    pub brand_color: String,
    pub label_short: String,
    #[props(default)]
    pub label_line2: String,
    #[props(default)]
    pub icon_text: String,
    /// Stamp size in px (from TOML layout.stamp_size).
    #[props(default = 32)]
    pub size: u16,
}

/// A single certification stamp — tiny inline SVG badge, config-driven.
#[component]
pub fn CertBadge(props: CertBadgeProps) -> Element {
    let sz = props.size.to_string();
    let vb = format!("0 0 {sz} {sz}");
    let half = (props.size / 2).to_string();
    let rx = (props.size / 8).to_string();
    let has_line2 = !props.label_line2.is_empty();

    // Font sizes proportional to stamp size
    let fs_main = (props.size as f32 * 0.30).round() as u16;
    let fs_sub = (props.size as f32 * 0.20).round() as u16;
    let y_main: u16 = if has_line2 {
        (props.size as f32 * 0.45).round() as u16
    } else {
        (props.size as f32 * 0.58).round() as u16
    };
    let y_sub: u16 = (props.size as f32 * 0.72).round() as u16;

    rsx! {
        div {
            class: "cert-stamp",
            role: "listitem",
            title: "{props.name}",

            svg {
                class: "cert-stamp-svg",
                width: "{sz}",
                height: "{sz}",
                view_box: "{vb}",
                "aria-hidden": "true",
                rect {
                    x: "0", y: "0",
                    width: "{sz}", height: "{sz}",
                    rx: "{rx}",
                    fill: "{props.brand_color}",
                }
                text {
                    x: "{half}", y: "{y_main}",
                    "text-anchor": "middle",
                    "font-family": "Arial, sans-serif",
                    "font-size": "{fs_main}",
                    "font-weight": "bold",
                    fill: "white",
                    "{props.label_short}"
                }
                if has_line2 {
                    text {
                        x: "{half}", y: "{y_sub}",
                        "text-anchor": "middle",
                        "font-family": "Arial, sans-serif",
                        "font-size": "{fs_sub}",
                        "font-weight": "bold",
                        fill: "rgba(255,255,255,0.85)",
                        "{props.label_line2}"
                    }
                }
            }

            span { class: "cert-stamp-label", "{props.name}" }
        }
    }
}

/// The full certification strip — reads layout + certs from TOML config.
#[component]
pub fn CertStrip() -> Element {
    let config = crate::config::cert_config();
    let layout = &config.layout;
    let certs = &config.certs;

    let dir = if layout.direction == "vertical" {
        "column"
    } else {
        "row"
    };
    let gap = format!("{}px", layout.gap);
    let wrap = if layout.rows > 1 { "wrap" } else { "nowrap" };

    rsx! {
        div {
            class: "hero-cert-stamps",
            role: "list",
            "aria-label": "Certifications held by Alashore Marine Exports",
            style: "flex-direction:{dir};gap:{gap};flex-wrap:{wrap};",

            for cert in certs.iter() {
                CertBadge {
                    key: "{cert.id}",
                    id: cert.id.clone(),
                    name: cert.name.clone(),
                    brand_color: cert.brand_color.clone(),
                    label_short: cert.label_short.clone(),
                    label_line2: cert.label_line2.clone(),
                    icon_text: cert.icon_text.clone(),
                    size: layout.stamp_size,
                }
            }
        }
    }
}
