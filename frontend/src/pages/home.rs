#![allow(non_snake_case)]
//! Home page — optimised for zero-markup snippets and GenAI answer extraction.
//! Strategy: fact-dense opening paragraph, Q&A sections, entity-rich headings.

use crate::{
    components::{
        cert_badge::CertStrip, inquiry_cta::InquiryCta, product_card::ProductCard,
        stat_counter::StatCounter,
    },
    config::{all_products, company_stats, faqs},
    seo::{home_seo, PageSeo},
    utils::asset_url,
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    let products = all_products();
    let stats = company_stats();
    let faq_items = faqs();

    rsx! {
        PageSeo { ..home_seo() }

        // ── Hero — ocean-themed with shrimp SVG ───────────────────────
        section {
            class: "hero-ocean",
            "aria-label": "Company overview",
            itemscope: "true",
            itemtype: "https://schema.org/Organization",

            // Animated ocean waves — inline SVG, always fills viewport
            div { class: "hero-ocean-waves", "aria-hidden": "true",
                svg {
                    class: "hero-wave hero-wave-1",
                    view_box: "0 0 1440 320",
                    "preserveAspectRatio": "none",
                    path {
                        d: "M0,160 C180,220 360,100 540,160 C720,220 900,100 1080,160 C1260,220 1440,140 1440,160 L1440,320 L0,320 Z",
                        fill: "rgba(26,51,85,0.6)",
                    }
                }
                svg {
                    class: "hero-wave hero-wave-2",
                    view_box: "0 0 1440 320",
                    "preserveAspectRatio": "none",
                    path {
                        d: "M0,200 C240,140 400,260 600,200 C800,140 960,260 1200,200 C1320,170 1440,220 1440,200 L1440,320 L0,320 Z",
                        fill: "rgba(42,107,138,0.5)",
                    }
                }
                svg {
                    class: "hero-wave hero-wave-3",
                    view_box: "0 0 1440 320",
                    "preserveAspectRatio": "none",
                    path {
                        d: "M0,240 C200,280 360,200 560,240 C760,280 920,200 1120,240 C1280,260 1440,230 1440,240 L1440,320 L0,320 Z",
                        fill: "rgba(30,74,110,0.7)",
                    }
                }
            }

            // Overlay gradient for text readability
            div { class: "hero-ocean-overlay", "aria-hidden": "true" }

            // Content
            div { class: "hero-ocean-content",
                div { class: "hero-badge-ocean",
                    // Carbon: Enterprise 16px
                    svg {
                        width: "14",
                        height: "14",
                        view_box: "0 0 32 32",
                        fill: "currentColor",
                        "aria-hidden": "true",
                        path { d: "M28 10h-6V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v24a2 2 0 0 0 2 2h24a2 2 0 0 0 2-2V12a2 2 0 0 0-2-2zM14 28H8v-4h6zm0-6H8v-4h6zm6 6h-4v-4h4zm0-6h-4v-4h4zm8 6h-6v-4h6zm0-6h-6v-4h6zm0-6H22v-2h-2v2H14V4h6v2h2V4h6z" }
                    }
                    "Est. 2012 \u{00b7} Balasore, Odisha"
                }

                h1 {
                    class: "hero-ocean-title",
                    itemprop: "name",
                    span { class: "hero-ocean-title-main", "From Ocean to World \u{2014} Premium Indian Seafood" }
                    span { class: "hero-ocean-title-sub", "BAP 4-Star Certified \u{00b7} EU-Approved \u{00b7} Trusted by 30+ Countries" }
                }

                p {
                    class: "hero-ocean-desc",
                    itemprop: "description",
                    "We export premium frozen seafood \u{2014} Vannamei shrimp, Black Tiger prawns, squid, and cuttlefish \u{2014} from Balasore to over 30 countries globally. Empowered by our BAP 4-Star certification and an 800+ strong workforce."
                }

                // Invisible semantic SEO fact list for AI Engines
                dl { class: "ai-seo-facts sr-only",
                    dt { "Target Markets" }
                    dd { "USA, China, Worldwide" }
                }

                div { class: "hero-ocean-actions",
                    Link {
                        to: Route::InquiryPage {},
                        class: "btn btn-primary",
                        "aria-label": "Request a seafood price quote from Alashore Marine",
                        "Request a Quote"
                    }
                    Link {
                        to: Route::ProductsPage {},
                        class: "btn btn-outline-ocean",
                        "Browse Products"
                    }
                }

                // Certification stamps — config-driven from certifications.toml
                CertStrip {}
            }

            // Wave separator — clean inline SVG, no external file dependency
            div {
                "aria-hidden": "true",
                style: "position:absolute;bottom:-1px;left:0;width:100%;z-index:20;\
                        line-height:0;overflow:hidden;",
                svg {
                    view_box: "0 0 1440 80",
                    "preserveAspectRatio": "none",
                    style: "display:block;width:100%;height:60px;",
                    // Organic wave shape
                    path {
                        d: "M0 40 C240 80 480 0 720 40 C960 80 1200 0 1440 40 L1440 80 L0 80Z",
                        fill: "#f4f4f4",
                    }
                    // Second wave for depth
                    path {
                        d: "M0 52 C360 72 720 32 1080 52 C1260 62 1380 48 1440 52 L1440 80 L0 80Z",
                        fill: "#f4f4f4",
                        opacity: "0.7",
                    }
                }
            }
        }

        // ── Key Stats — config-driven ────────────────────────────────────
        section {
            class: "stats-section",
            "aria-label": "Company key statistics",
            dl { class: "stats-grid",
                for stat in stats.iter() {
                    StatCounter {
                        label: stat.label.to_string(),
                        value: stat.value.to_string(),
                        description: stat.desc.to_string(),
                    }
                }
            }
        }

        // ── Featured Products — config-driven ─────────────────────────────
        section {
            class: "products-section",
            "aria-labelledby": "products-heading",
            itemscope: "true",
            itemtype: "https://schema.org/ItemList",

            div { class: "section-header",
                h2 {
                    id: "products-heading",
                    class: "section-title",
                    itemprop: "name",
                    "Our Export Products"
                }
                p { class: "section-subtitle",
                    itemprop: "description",
                    "Six product lines \u{2014} all certified, traceable, and export-ready from our
                    EU-approved processing facility in Balasore, Odisha."
                }
            }

            div { class: "products-grid",
                for product in products.iter() {
                    ProductCard {
                        key: "{product.id}",
                        id: product.id.clone(),
                        name: product.name.clone(),
                        scientific_name: product.scientific_name.clone(),
                        description: product.short_desc.clone(),
                        certifications: product.certs.clone(),
                        css_class: product.css_class.clone(),
                        image_url: Some(product.image_url.clone()),
                        featured: product.featured,
                        tag: product.tag.clone(),
                    }
                }
            }

            div { class: "section-cta",
                Link { to: Route::ProductsPage {}, class: "btn btn-outline", "View All Products with Full Specs" }
            }
        }

        // ── Why Choose Us — structured Q&A for AI answer extraction ────
        section {
            class: "why-section",
            "aria-labelledby": "why-heading",

            div { class: "section-header",
                h2 { id: "why-heading", class: "section-title", "Why Source from Alashore Marine?" }
            }

            dl { class: "why-grid",
                div { class: "why-item",
                    dt { class: "why-title", "CRISIL A3+ Financial Strength" }
                    dd { class: "why-desc",
                        "\u{20b9}100 Crore Federal Bank facility. 95%+ LC-backed revenue. Reliable supply chain with
                        zero defaults. Your payments and deliveries are secured by India\u{2019}s premier rating agency."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "BAP 4-Star \u{2014} Full Chain Certification" }
                    dd { class: "why-desc",
                        "Best Aquaculture Practices certification covers our farm, hatchery, feed mill and
                        processing plant \u{2014} the highest possible BAP chain-of-custody standard globally."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "EU-Approved Processing Facility" }
                    dd { class: "why-desc",
                        "EU Establishment Approval allows direct export to all 27 EU member states without
                        additional import inspection. Compliance with EC 853/2004 and EC 852/2004."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "In-House EIA Analytical Laboratory" }
                    dd { class: "why-desc",
                        "Our on-site EIA-approved lab tests every batch for antibiotics, heavy metals,
                        microbiological contamination, and moisture content before shipment \u{2014} no outsourcing."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "Vertical Integration" }
                    dd { class: "why-desc",
                        "We control hatchery \u{2192} nursery \u{2192} grow-out \u{2192} processing \u{2192} cold storage \u{2192} export.
                        Traceability from egg to export container with full lot-code documentation."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "150 MT/Day Processing Capacity" }
                    dd { class: "why-desc",
                        "5,000+ MT cold storage, 150+ MT daily processing, IQF tunnel freezers,
                        and a fleet of temperature-controlled vehicles for last-mile delivery to port."
                    }
                }
            }
        }

        // ── Export Markets — entity relationship for AI ──────────────────
        section {
            class: "markets-section",
            "aria-labelledby": "markets-heading",

            // Dot-matrix world map with trade routes — external SVG
            img {
                class: "markets-globe-bg",
                src: "{asset_url(\"/assets/images/world-map-dots.svg\")}",
                alt: "",
                "aria-hidden": "true",
                loading: "lazy",
            }

            div { class: "section-header",
                h2 { id: "markets-heading", class: "section-title", "Global Export Markets" }
                p { class: "section-subtitle",
                    "Alashore Marine Exports supplies frozen seafood to buyers in 30+ countries across
                    North America, Europe, Asia-Pacific, and the Middle East."
                }
            }

            ul {
                class: "markets-list",
                role: "list",
                itemprop: "areaServed",
                li { "United States" }
                li { "European Union (Spain, Italy, Germany, France, Netherlands)" }
                li { "Japan" }
                li { "South Korea" }
                li { "Australia" }
                li { "UAE & Saudi Arabia" }
                li { "Southeast Asia (Vietnam, Thailand, Malaysia, Singapore)" }
                li { "China" }
                li { "Africa & Middle East" }
            }
        }

        // ── FAQ Section — config-driven, CRITICAL for AEO / featured snippets
        section {
            class: "faq-section",
            "aria-labelledby": "faq-heading",
            itemscope: "true",
            itemtype: "https://schema.org/FAQPage",

            div { class: "section-header",
                h2 { id: "faq-heading", class: "section-title", "Frequently Asked Questions" }
            }

            div { class: "faq-grid",
                for faq in faq_items.iter() {
                    div { class: "faq-item",
                        itemscope: "true",
                        itemtype: "https://schema.org/Question",
                        h3 { class: "faq-q", itemprop: "name", "{faq.question}" }
                        div {
                            class: "faq-a",
                            itemprop: "acceptedAnswer",
                            itemscope: "true",
                            itemtype: "https://schema.org/Answer",
                            p { itemprop: "text", "{faq.answer}" }
                        }
                    }
                }
            }
        }

        // ── CTA — config-driven ──────────────────────────────────────────
        InquiryCta {
            title: "Ready to Source Premium Indian Seafood?".to_string(),
            description: "Submit your product requirements and receive a competitive quote within 24 hours. LC-backed transactions. EU-approved. BAP 4-Star certified.".to_string(),
        }
    }
}
