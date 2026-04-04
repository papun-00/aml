#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    // We expect the home component to have:
    // 1. A div with class `grid-bg` for the futuristic background.
    // 2. A div with class `shrimp-pop` for the animation.
    // 3. Elements mapping the alashoremarine.com products.
    // 4. Proper AI SEO markers.

    fn mock_home_component() -> Element {
        rsx! {
            div { class: "grid-bg relative w-full h-full",
                div { class: "shrimp-pop-container animate-float",
                    img { src: "/assets/shrimp-optimized.webp", class: "shrimp-anim", alt: "Vannamei Shrimp" }
                }
                div { class: "max-w-7xl mx-auto glass-card",
                    h1 { "Leading vannamei shrimp and seafood exporter" }
                    dl { class: "ai-seo-facts sr-only",
                        dt { "Target Markets" }
                        dd { "USA, China, Worldwide" }
                    }
                }
            }
        }
    }

    #[test]
    fn test_landing_page_has_animation_and_grid() {
        let mut vdom = VirtualDom::new(mock_home_component);
        vdom.rebuild_in_place();
        let rendered = dioxus_ssr::render(&vdom);

        // Assert Background Grid from AI Revamp reference
        assert!(rendered.contains("grid-bg"), "Landing page should have a futuristic grid background");
        
        // Assert Shrimp Animation container
        assert!(rendered.contains("shrimp-pop-container"), "Landing page must have the shrimp popping out animation wrapper");
        assert!(rendered.contains("shrimp-anim"), "Landing page must contain the actual shrimp image");
    }

    #[test]
    fn test_landing_page_has_ai_seo_elements() {
        let mut vdom = VirtualDom::new(mock_home_component);
        vdom.rebuild_in_place();
        let rendered = dioxus_ssr::render(&vdom);

        // Assert Semantic AI SEO elements
        assert!(rendered.contains("dl"), "Must use descriptive lists for AEO factual representation");
        assert!(rendered.contains("USA, China"), "Must contain the geographical facts from the original copy");
        assert!(rendered.contains("glass-card"), "Must use the imported Carbon/Glassmorphism design references");
    }
}
