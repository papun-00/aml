#![allow(non_snake_case)]
//! Ascending staircase timeline — light theme, colorful gradient cards.
//!
//! Cards rise gently along an SVG curve on a light purple gradient background.
//! Section heading uses `.section-title` class for page-wide font consistency.
//! Rotating word animation in the heading. IBM Carbon 32px icons.
//! Config-driven, all card styling inline, section uses CSS classes for uniformity.

use dioxus::prelude::*;
use crate::config::GrowthMilestone;

// ── Accent colors for card variety ────────────────────────────────────────
const ACCENT_BLUE: &str = "#0f62fe";
const ACCENT_TEAL: &str = "#009d9a";
const ACCENT_PURPLE: &str = "#6929c4";

fn carbon_icon(icon: &str) -> &'static str {
    match icon {
        "anchor" => "M28 12h-2.1A9.94 9.94 0 0 0 18 4.1V2h-4v2.1A9.94 9.94 0 0 0 6.1 12H4v2h2.1A9.94 9.94 0 0 0 14 21.9V30h4v-8.1A9.94 9.94 0 0 0 25.9 14H28zM16 20a6 6 0 1 1 6-6 6 6 0 0 1-6 6z",
        "globe" => "M16 2A14 14 0 1 0 30 16 14 14 0 0 0 16 2zm12 13h-6a24.2 24.2 0 0 0-2.8-10.4A12 12 0 0 1 28 15zM16 4a10.9 10.9 0 0 1 3.2 1 22 22 0 0 1 3 10H9.8a22 22 0 0 1 3-10A10.9 10.9 0 0 1 16 4z",
        "star" => "M16 2l4.2 8.6L30 12l-7 6.8L24.6 28 16 23.4 7.4 28 9 18.8 2 12l9.8-1.4z",
        "shield" => "M16 2L4 7v9c0 8.4 5.1 16.2 12 18 6.9-1.8 12-9.6 12-18V7zm0 25.7C10.3 25.9 6 19.1 6 16V8.4l10-4.3 10 4.3V16c0 3.1-4.3 9.9-10 11.7z",
        "refresh" => "M12 10H6.78A11 11 0 0 1 27 16h2A13 13 0 0 0 6 7.68V4H4v8h8zm8 12h5.22A11 11 0 0 1 5 16H3a13 13 0 0 0 23 8.32V28h2V20h-8z",
        "trending-up" => "M28 6h-8v2h5.6L18 15.6l-6-6L2 19.6 3.4 21l8-8 6 6L26 10.4V16h2z",
        _ => "M16 2A14 14 0 1 0 30 16 14 14 0 0 0 16 2zm0 26A12 12 0 1 1 28 16 12 12 0 0 1 16 28z",
    }
}

fn accent_for(i: usize, highlight: bool) -> &'static str {
    if highlight { return ACCENT_PURPLE; }
    match i % 3 { 0 => ACCENT_BLUE, 1 => ACCENT_TEAL, _ => ACCENT_PURPLE }
}

/// CSS class suffix for card gradient variant — applied via `.jcard-v{n}`.
fn card_variant_class(i: usize, highlight: bool) -> &'static str {
    if highlight { return "jcard-highlight"; }
    match i % 4 {
        0 => "jcard-v0",
        1 => "jcard-v1",
        2 => "jcard-v2",
        _ => "jcard-v3",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct GrowthTimelineProps {
    pub milestones: Vec<GrowthMilestone>,
    #[props(default = "Our Growth Journey".to_string())]
    pub title: String,
    #[props(default = "From a single processing unit in Balasore to 30+ global markets — a decade of relentless growth.".to_string())]
    pub subtitle: String,
}

/// Node position on the ascending curve. Shallow ~15° incline.
fn node_pos(i: usize, count: usize) -> (f64, f64) {
    let t = if count <= 1 { 0.5 } else { i as f64 / (count as f64 - 1.0) };
    let x = 60.0 + t * 880.0;
    let y = 320.0 - t * 130.0; // 320..190 — gentle rise with more room above
    (x, y)
}

#[component]
pub fn GrowthTimeline(props: GrowthTimelineProps) -> Element {
    let total = props.milestones.len();
    let mut active = use_signal(|| total.saturating_sub(1));

    use_effect(move || {
        spawn(async move {
            let d = "await new Promise(r => setTimeout(r, 250));";
            let _ = document::eval(d).await;
            let _ = document::eval(NAV_JS);
        });
    });

    // Build SVG curve
    let curve = {
        let mut d = String::new();
        for i in 0..total {
            let (x, y) = node_pos(i, total);
            if i == 0 {
                d.push_str(&format!("M{} {}", x - 30.0, y + 10.0));
                d.push_str(&format!(" S{x} {y} {x} {y}"));
            } else {
                let (px, _py) = node_pos(i - 1, total);
                let cx = px + (x - px) * 0.5;
                d.push_str(&format!(" C{cx} {_py} {cx} {y} {x} {y}"));
            }
        }
        let (lx, ly) = node_pos(total - 1, total);
        d.push_str(&format!(" L{} {}", lx + 40.0, ly - 15.0));
        d
    };

    rsx! {
        style { {KEYFRAMES} }

        section {
            id: "growth-journey",
            class: "journey-section",
            "aria-labelledby": "journey-heading",
            tabindex: "0",

            // ── Header — uses same CSS classes as other sections ──────
            div { class: "section-header",
                h2 {
                    id: "journey-heading",
                    class: "section-title",
                    "Our Growth Journey"
                }
                // Rotating words as a separate line below the title
                div {
                    style: "display:flex;align-items:center;gap:0.5rem;margin-top:0.5rem;\
                            margin-bottom:1rem;height:2rem;",
                    span {
                        style: "position:relative;display:inline-block;width:8rem;height:1.75rem;\
                                overflow:hidden;",
                        span {
                            style: "position:absolute;left:0;top:0;width:100%;color:{ACCENT_BLUE};\
                                    font-size:1.25rem;font-weight:600;\
                                    animation:jword 12s infinite 0s;opacity:0;transform:translateY(100%);",
                            "Growth"
                        }
                        span {
                            style: "position:absolute;left:0;top:0;width:100%;color:{ACCENT_TEAL};\
                                    font-size:1.25rem;font-weight:600;\
                                    animation:jword 12s infinite 3s;opacity:0;transform:translateY(100%);",
                            "Vision"
                        }
                        span {
                            style: "position:absolute;left:0;top:0;width:100%;color:{ACCENT_PURPLE};\
                                    font-size:1.25rem;font-weight:600;\
                                    animation:jword 12s infinite 6s;opacity:0;transform:translateY(100%);",
                            "Impact"
                        }
                        span {
                            style: "position:absolute;left:0;top:0;width:100%;color:{ACCENT_BLUE};\
                                    font-size:1.25rem;font-weight:600;\
                                    animation:jword 12s infinite 9s;opacity:0;transform:translateY(100%);",
                            "Trust"
                        }
                    }
                }
                p { class: "section-subtitle", "{props.subtitle}" }
            }

            // ── Staircase viewport ──────────────────────────
            div {
                id: "staircase-vp",
                style: "position:relative;max-width:1000px;margin:0 auto;height:420px;",

                // SVG curve + nodes
                svg {
                    "aria-hidden": "true",
                    view_box: "0 0 1000 420",
                    "preserveAspectRatio": "xMidYMid meet",
                    style: "position:absolute;inset:0;width:100%;height:100%;z-index:1;\
                            pointer-events:none;",
                    defs {
                        linearGradient {
                            id: "curve-g",
                            x1: "0%", y1: "100%", x2: "100%", y2: "0%",
                            stop { offset: "0%", stop_color: "#93c5fd", stop_opacity: "0.3" }
                            stop { offset: "100%", stop_color: "#c4b5fd", stop_opacity: "0.5" }
                        }
                    }
                    path {
                        d: "{curve}",
                        fill: "none",
                        stroke: "url(#curve-g)",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                    }
                    // Arrow tip
                    {
                        let (lx, ly) = node_pos(total - 1, total);
                        let ax = lx + 40.0;
                        let ay = ly - 15.0;
                        rsx! {
                            polygon {
                                points: "{ax},{ay} {ax - 10.0},{ay - 7.0} {ax - 7.0},{ay + 7.0}",
                                fill: "#c4b5fd", opacity: "0.6",
                            }
                        }
                    }
                    // Node circles
                    for i in 0..total {
                        {
                            let (cx, cy) = node_pos(i, total);
                            let is_a = active() == i;
                            let r = if is_a { "7" } else { "5" };
                            let fill = if is_a { accent_for(i, props.milestones[i].highlight) } else { "white" };
                            let sw = if is_a { "3" } else { "2" };
                            rsx! {
                                circle {
                                    cx: "{cx}", cy: "{cy}", r: "{r}",
                                    fill: "{fill}",
                                    stroke: "{accent_for(i, props.milestones[i].highlight)}",
                                    stroke_width: "{sw}",
                                    style: "transition:all 0.3s;",
                                }
                            }
                        }
                    }
                }

                // Cards above curve
                for (i, m) in props.milestones.iter().enumerate() {
                    {
                        let (nx, ny) = node_pos(i, total);
                        let is_a = active() == i;
                        let accent = accent_for(i, m.highlight);
                        let variant = card_variant_class(i, m.highlight);

                        let card_w = if is_a { 210.0 } else { 130.0 };
                        let left_pct = (nx - card_w / 2.0) / 10.0;
                        let bottom_from = ny - 12.0;

                        let active_cls = if is_a { "jcard-active" } else { "" };
                        let z = if is_a { "10" } else { "2" };

                        rsx! {
                            div {
                                key: "{m.year}",
                                class: "jcard {variant} {active_cls}",
                                "data-ci": "{i}",
                                onclick: move |_| active.set(i),
                                style: "position:absolute;\
                                        left:{left_pct}%;bottom:calc(100% - {bottom_from}px);\
                                        width:{card_w}px;z-index:{z};",

                                // Year + icon
                                {
                                    let ico_size = if is_a { "18" } else { "14" };
                                    rsx! {
                                        div {
                                            class: "jcard-head",
                                            span { class: "jcard-year", "{m.year}" }
                                            svg {
                                                width: "{ico_size}", height: "{ico_size}", view_box: "0 0 32 32",
                                                fill: "{accent}", opacity: "0.7", "aria-hidden": "true",
                                                path { d: "{carbon_icon(m.icon)}" }
                                            }
                                        }
                                    }
                                }

                                h3 { class: "jcard-title", "{m.title}" }

                                // Metric
                                if let (Some(label), Some(value)) = (m.metric_label, m.metric_value) {
                                    div { class: "jcard-metric",
                                        span { class: "jcard-metric-label", "{label}:" }
                                        span {
                                            class: "jcard-metric-value",
                                            style: "color:{accent};",
                                            "{value}"
                                        }
                                    }
                                    div { class: "jcard-bar",
                                        div {
                                            class: "jcard-bar-fill",
                                            style: "background:{accent};\
                                                    width:{(i + 1) as f64 / total as f64 * 100.0}%;",
                                        }
                                    }
                                }

                                // Description (active only)
                                if is_a {
                                    p {
                                        class: "jcard-desc",
                                        "{m.description}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Navigation dots
            div { class: "jnav-dots",
                for i in 0..total {
                    {
                        let is_a = active() == i;
                        let accent = accent_for(i, props.milestones[i].highlight);
                        let cls = if is_a { "jnav-dot jnav-dot-active" } else { "jnav-dot" };
                        rsx! {
                            button {
                                class: "{cls}",
                                "aria-label": "Milestone {props.milestones[i].year}",
                                style: if is_a { format!("background:{accent};width:1.5rem;") } else { String::new() },
                                onclick: move |_| active.set(i),
                            }
                        }
                    }
                }
            }
        }
    }
}

const KEYFRAMES: &str = r#"
@keyframes jword {
    0%       { opacity: 0; transform: translateY(100%); }
    4%       { opacity: 1; transform: translateY(0); }
    21%      { opacity: 1; transform: translateY(0); }
    25%      { opacity: 0; transform: translateY(-100%); }
    100%     { opacity: 0; transform: translateY(-100%); }
}
@keyframes jfade {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
}
"#;

const NAV_JS: &str = r#"
(function() {
    var s = document.getElementById('growth-journey');
    if (!s) return;
    var cards = s.querySelectorAll('[data-ci]');
    var n = cards.length;
    s.addEventListener('keydown', function(e) {
        var curr = s.querySelector('[data-ci][style*="z-index:10"]');
        var idx = curr ? parseInt(curr.dataset.ci) : 0;
        if (e.key==='ArrowRight'||e.key==='ArrowDown') {
            idx = Math.min(idx+1, n-1); e.preventDefault();
        } else if (e.key==='ArrowLeft'||e.key==='ArrowUp') {
            idx = Math.max(idx-1, 0); e.preventDefault();
        } else return;
        cards[idx].click();
    });

    if (window.innerWidth < 768) {
        var vp = document.getElementById('staircase-vp');
        if (!vp) return;
        vp.style.height = 'auto';
        vp.style.display = 'flex';
        vp.style.flexDirection = 'column';
        vp.style.gap = '0.75rem';
        var svg = vp.querySelector('svg');
        if (svg) svg.style.display = 'none';
        cards.forEach(function(c) {
            c.style.position = 'relative';
            c.style.left = 'auto';
            c.style.bottom = 'auto';
            c.style.width = '100%';
            c.style.maxWidth = '320px';
            c.style.margin = '0 auto';
            c.style.opacity = '1';
            c.style.transform = 'none';
        });
    }
})();
"#;
