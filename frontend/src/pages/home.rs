#![allow(non_snake_case)]
//! Home page — optimised for zero-markup snippets and GenAI answer extraction.
//! Strategy: fact-dense opening paragraph, Q&A sections, entity-rich headings.

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    Route,
    seo::{PageSeo, home_seo},
    config::{all_products, company_stats, certifications, faqs},
    components::{
        product_card::ProductCard,
        stat_counter::StatCounter,
        cert_badge::CertBadge,
        inquiry_cta::InquiryCta,
    },
};

#[component]
pub fn HomePage() -> Element {
    let products = all_products();
    let stats = company_stats();
    let certs = certifications();
    let faq_items = faqs();

    rsx! {
        PageSeo { ..home_seo() }

        // ── Hero — semantic, crawlable ─────────────────────────────────
        section {
            class: "hero grid-bg min-h-screen flex items-center justify-center relative",
            "aria-label": "Company overview",
            itemscope: "true",
            itemtype: "https://schema.org/Organization",

            div { class: "hero-content relative z-10 glass-card p-12 max-w-7xl mx-auto w-full",
                div { class: "hero-badge mb-6 inline-block bg-carbon-blue-60 text-white px-3 py-1 text-sm font-mono tracking-widest uppercase", "Est. 2012 · Balasore, India" }

                // The Shrimp Animation Wrapper
                div { class: "shrimp-pop-container absolute -top-16 -right-16 w-64 h-64 animate-float hidden md:block z-50 pointer-events-none",
                    img {
                        src: "https://images.unsplash.com/photo-1544551763-46a013bb70d5?auto=format&fit=crop&q=80&w=400",
                        class: "shrimp-anim animate-shrimp-pop w-full h-full object-contain filter drop-shadow-glow",
                        alt: "Vannamei Shrimp"
                    }
                }

                h1 {
                    class: "hero-title text-6xl font-light text-white mb-6",
                    itemprop: "name",
                    span { class: "hero-title-main block mb-2", "Alashore Marine Exports" }
                    span { class: "hero-title-sub block text-3xl text-carbon-blue-30", "India's Premier Frozen Seafood Exporter" }
                }

                p {
                    class: "hero-description text-lg text-gray-300 max-w-2xl leading-relaxed mb-8",
                    itemprop: "description",
                    "We export premium frozen seafood — Vannamei shrimp, Black Tiger prawns, squid, and cuttlefish — from Balasore to over 30 countries globally. Empowered by our BAP 4-Star certification and an 800+ strong workforce."
                }

                // Invisible semantic SEO fact list for AI Engines
                dl { class: "ai-seo-facts sr-only",
                    dt { "Target Markets" }
                    dd { "USA, China, Worldwide" }
                }

                div { class: "hero-actions",
                    Link {
                        to: Route::InquiryPage {},
                        class: "btn btn-primary",
                        "aria-label": "Request a seafood price quote from Alashore Marine",
                        "Request a Quote"
                    }
                    Link {
                        to: Route::ProductsPage {},
                        class: "btn btn-ghost",
                        "Browse Products"
                    }
                }

                // Certification logos row — config-driven
                div {
                    class: "hero-certs",
                    role: "list",
                    "aria-label": "Certifications held by Alashore Marine Exports",
                    for cert in certs.iter() {
                        CertBadge { name: cert.name.to_string(), icon: cert.icon.to_string() }
                    }
                }
            }

            div { class: "hero-visual", "aria-hidden": "true",
                div { class: "hero-globe" }
                div { class: "hero-wave wave-1" }
                div { class: "hero-wave wave-2" }
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
                    "Six product lines — all certified, traceable, and export-ready from our
                    EU-approved processing facility in Balasore, Odisha."
                }
            }

            div { class: "products-grid",
                for product in products.iter() {
                    ProductCard {
                        key: "{product.id}",
                        id: product.id.to_string(),
                        name: product.name.to_string(),
                        scientific_name: product.scientific_name.to_string(),
                        description: product.short_desc.to_string(),
                        certifications: product.certs.iter().map(|c| c.to_string()).collect(),
                        css_class: product.css_class.to_string(),
                        image_url: Some(product.image_url.to_string()),
                        featured: product.featured,
                        tag: product.tag.map(|t| t.to_string()),
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
                        "₹100 Crore Federal Bank facility. 95%+ LC-backed revenue. Reliable supply chain with
                        zero defaults. Your payments and deliveries are secured by India's premier rating agency."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "BAP 4-Star — Full Chain Certification" }
                    dd { class: "why-desc",
                        "Best Aquaculture Practices certification covers our farm, hatchery, feed mill and
                        processing plant — the highest possible BAP chain-of-custody standard globally."
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
                        microbiological contamination, and moisture content before shipment — no outsourcing."
                    }
                }
                div { class: "why-item",
                    dt { class: "why-title", "Vertical Integration" }
                    dd { class: "why-desc",
                        "We control hatchery → nursery → grow-out → processing → cold storage → export.
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
                li { "🇺🇸 United States" }
                li { "🇪🇺 European Union (Spain, Italy, Germany, France, Netherlands)" }
                li { "🇯🇵 Japan" }
                li { "🇰🇷 South Korea" }
                li { "🇦🇺 Australia" }
                li { "🇦🇪 UAE & Saudi Arabia" }
                li { "🌏 Southeast Asia (Vietnam, Thailand, Malaysia, Singapore)" }
                li { "🇨🇳 China" }
                li { "🌍 Africa & Middle East" }
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
