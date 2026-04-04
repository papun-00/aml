#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{Route, seo::{PageSeo, products_seo}};

#[derive(Clone, PartialEq)]
enum Filter { All, Shrimp, Fish, Cephalopods, Dried }

#[component]
pub fn ProductsPage() -> Element {
    let mut active = use_signal(|| Filter::All);

    rsx! {
        PageSeo { ..products_seo() }

        section { class: "page-hero",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "Products" }
                    }
                }
                h1 { class: "page-title", "Our Export Product Range" }
                p { class: "page-subtitle",
                    "All products are certified, traceable, and processed at our EU-approved
                    facility in Balasore, Odisha. Minimum order quantities from 2 MT."
                }
            }
        }

        section { class: "products-page-section",
            // Filter tabs
            div { class: "filter-tabs", role: "tablist", "aria-label": "Product category filter",
                button {
                    class: if active() == Filter::All { "filter-tab active" } else { "filter-tab" },
                    role: "tab",
                    "aria-selected": "{active() == Filter::All}",
                    onclick: move |_| active.set(Filter::All),
                    "All Products"
                }
                button {
                    class: if active() == Filter::Shrimp { "filter-tab active" } else { "filter-tab" },
                    role: "tab",
                    "aria-selected": "{active() == Filter::Shrimp}",
                    onclick: move |_| active.set(Filter::Shrimp),
                    "Shrimp & Prawns"
                }
                button {
                    class: if active() == Filter::Fish { "filter-tab active" } else { "filter-tab" },
                    role: "tab",
                    "aria-selected": "{active() == Filter::Fish}",
                    onclick: move |_| active.set(Filter::Fish),
                    "Fish"
                }
                button {
                    class: if active() == Filter::Cephalopods { "filter-tab active" } else { "filter-tab" },
                    role: "tab",
                    "aria-selected": "{active() == Filter::Cephalopods}",
                    onclick: move |_| active.set(Filter::Cephalopods),
                    "Cephalopods"
                }
                button {
                    class: if active() == Filter::Dried { "filter-tab active" } else { "filter-tab" },
                    role: "tab",
                    "aria-selected": "{active() == Filter::Dried}",
                    onclick: move |_| active.set(Filter::Dried),
                    "Dried"
                }
            }

            // Product grid — all with semantic schema.org markup
            div {
                class: "products-grid products-grid-full",
                role: "tabpanel",
                "aria-live": "polite",
                itemscope: "true",
                itemtype: "https://schema.org/ItemList",

                if active() == Filter::All || active() == Filter::Shrimp {
                    ProductRow {
                        id: "vannamei-shrimp",
                        name: "Vannamei Whiteleg Shrimp",
                        scientific: "Litopenaeus vannamei",
                        category: "Shrimp & Prawns",
                        hs_code: "0306.17",
                        description: "BAP 4-Star certified aquaculture shrimp from Odisha. Sizes U/10 to 50/60. HOSO, HLSO, PTO, PD, Butterfly, EZ-Peel, IQF formats. Shelf life 24 months at -18°C.",
                        certs: vec!["BAP 4★", "ASC", "BRC AA", "HACCP", "EU Approved"],
                        markets: vec!["USA", "EU", "Japan", "South Korea", "Australia"],
                        min_order: "20,000 kg",
                        featured: true,
                    }
                    ProductRow {
                        id: "black-tiger-shrimp",
                        name: "Black Tiger Prawn",
                        scientific: "Penaeus monodon",
                        category: "Shrimp & Prawns",
                        hs_code: "0306.16",
                        description: "Wild-caught and farm-raised giant tiger prawns from Bay of Bengal. Sizes U/8 to 16/20. HOSO, HLSO, PTO, Sashimi grade available.",
                        certs: vec!["MPEDA", "HACCP", "EU Approved", "FSSAI"],
                        markets: vec!["Japan", "USA", "UAE", "EU"],
                        min_order: "5,000 kg",
                        featured: true,
                    }
                }

                if active() == Filter::All || active() == Filter::Fish {
                    ProductRow {
                        id: "pink-perch",
                        name: "Pink Perch (Threadfin Bream)",
                        scientific: "Nemipterus japonicus",
                        category: "Fish",
                        hs_code: "0302.89",
                        description: "Mild white-fleshed Bay of Bengal fish. 100g–500g+ sizes. Whole round, HG, fillet skin-on, fillet skinless. Surimi-grade and retail-grade available.",
                        certs: vec!["MPEDA", "HACCP", "EU Approved", "FSSAI"],
                        markets: vec!["China", "Southeast Asia", "Middle East", "EU"],
                        min_order: "10,000 kg",
                        featured: false,
                    }
                }

                if active() == Filter::All || active() == Filter::Cephalopods {
                    ProductRow {
                        id: "squid",
                        name: "Indian Squid",
                        scientific: "Doryteuthis sibogae",
                        category: "Cephalopods",
                        hs_code: "0307.43",
                        description: "Wild-caught Bay of Bengal squid. Whole uncleaned, whole cleaned, tubes & tentacles, rings (6mm or 10mm), steaks. Frozen within 2 hours of catch.",
                        certs: vec!["MPEDA", "HACCP", "EU Approved"],
                        markets: vec!["Spain", "Italy", "South Korea", "Japan", "UAE"],
                        min_order: "10,000 kg",
                        featured: true,
                    }
                    ProductRow {
                        id: "cuttlefish",
                        name: "Cuttlefish",
                        scientific: "Sepia pharaonis",
                        category: "Cephalopods",
                        hs_code: "0307.99",
                        description: "Pharaoh cuttlefish from Bay of Bengal. Whole uncleaned, whole cleaned, tubes & tentacles. Ink sac intact available on request.",
                        certs: vec!["MPEDA", "HACCP", "EU Approved"],
                        markets: vec!["Spain", "Portugal", "Japan", "South Korea"],
                        min_order: "5,000 kg",
                        featured: false,
                    }
                }

                if active() == Filter::All || active() == Filter::Dried {
                    ProductRow {
                        id: "dried-shrimp",
                        name: "Sun-Dried Shrimp",
                        scientific: "Dried & Processed",
                        category: "Dried Seafood",
                        hs_code: "0306.99",
                        description: "Sun-dried shrimp with no preservatives. Low moisture. Whole, crushed, or powder formats. 12-month shelf life at +4°C. Food-grade HDPE packing.",
                        certs: vec!["FSSAI", "MPEDA", "HACCP"],
                        markets: vec!["Southeast Asia", "Middle East", "Africa", "USA"],
                        min_order: "2,000 kg",
                        featured: false,
                    }
                }
            }
        }

        section { class: "page-cta",
            h2 { "Need Custom Sizing or Private Label?" }
            p { "Our team can accommodate custom processing specifications, private label packaging,
                 and custom incoterms. Contact us to discuss your requirements." }
            div { class: "cta-actions",
                Link { to: Route::InquiryPage {}, class: "btn btn-primary", "Request a Quote" }
                Link { to: Route::ContactPage {}, class: "btn btn-outline", "Talk to Our Team" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ProductRowProps {
    id: &'static str,
    name: &'static str,
    scientific: &'static str,
    category: &'static str,
    hs_code: &'static str,
    description: &'static str,
    certs: Vec<&'static str>,
    markets: Vec<&'static str>,
    min_order: &'static str,
    featured: bool,
}

#[component]
fn ProductRow(props: ProductRowProps) -> Element {
    rsx! {
        article {
            class: if props.featured { "product-row featured" } else { "product-row" },
            itemprop: "itemListElement",
            itemscope: "true",
            itemtype: "https://schema.org/Product",

            div { class: "product-row-header",
                if props.featured {
                    span { class: "product-tag", "Featured" }
                }
                h3 { class: "product-row-name", itemprop: "name", "{props.name}" }
                p { class: "product-row-sci",
                    em { itemprop: "alternateName", "{props.scientific}" }
                    " · HS: "
                    code { itemprop: "identifier", "{props.hs_code}" }
                }
                span { class: "product-row-category", "{props.category}" }
            }

            p { class: "product-row-desc", itemprop: "description", "{props.description}" }

            div { class: "product-row-meta",
                div { class: "product-row-certs",
                    for cert in &props.certs {
                        span { class: "cert-pill-sm", "{cert}" }
                    }
                }
                div { class: "product-row-markets",
                    span { class: "meta-label", "Markets:" }
                    span { itemprop: "areaServed", "{props.markets.join(\" · \")}" }
                }
                div { class: "product-row-moq",
                    span { class: "meta-label", "Min Order:" }
                    span { itemprop: "offers", "{props.min_order}" }
                }
            }

            Link {
                to: Route::ProductDetailPage { id: props.id.to_string() },
                class: "product-row-cta",
                itemprop: "url",
                "Full Specifications & Inquiry →"
            }
        }
    }
}
