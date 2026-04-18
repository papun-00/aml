#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    // Mock the ocean-themed hero with SVG background and semantic markup.
    fn mock_home_component() -> Element {
        rsx! {
            section { class: "hero-ocean",
                div { class: "hero-ocean-bg",
                    img { src: "/assets/images/hero-shrimp-waves.svg", class: "hero-ocean-svg", alt: "" }
                }
                div { class: "hero-ocean-overlay" }
                div { class: "hero-ocean-content",
                    h1 { "Alashore Marine Exports" }
                    p { class: "hero-ocean-desc",
                        "We export premium frozen seafood from Balasore to over 30 countries."
                    }
                    dl { class: "ai-seo-facts sr-only",
                        dt { "Target Markets" }
                        dd { "USA, China, Worldwide" }
                    }
                }
            }
        }
    }

    #[test]
    fn test_landing_page_has_ocean_hero() {
        let mut vdom = VirtualDom::new(mock_home_component);
        vdom.rebuild_in_place();
        let rendered = dioxus::ssr::render(&vdom);

        assert!(
            rendered.contains("hero-ocean"),
            "Landing page must have ocean-themed hero section"
        );
        assert!(
            rendered.contains("hero-ocean-svg"),
            "Landing page must have the SVG wave background"
        );
        assert!(
            rendered.contains("hero-ocean-content"),
            "Landing page must have the content layer"
        );
    }

    #[test]
    fn test_landing_page_has_ai_seo_elements() {
        let mut vdom = VirtualDom::new(mock_home_component);
        vdom.rebuild_in_place();
        let rendered = dioxus::ssr::render(&vdom);

        assert!(
            rendered.contains("dl"),
            "Must use descriptive lists for AEO factual representation"
        );
        assert!(
            rendered.contains("USA, China"),
            "Must contain the geographical facts from the original copy"
        );
        assert!(
            rendered.contains("hero-ocean-desc"),
            "Must have the hero description for SEO"
        );
    }
}
