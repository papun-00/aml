#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{Route, seo::{PageSeo, product_seo}};

#[derive(Clone, PartialEq)]
struct ProductData {
    id: &'static str,
    name: &'static str,
    scientific: &'static str,
    category: &'static str,
    hs_code: &'static str,
    description: &'static str,
    long_desc: &'static str,
    sizes: &'static str,
    forms: &'static [&'static str],
    shelf_life: &'static str,
    storage_temp: &'static str,
    glaze: &'static str,
    certs: &'static [&'static str],
    markets: &'static [&'static str],
    min_order: &'static str,
    incoterms: &'static [&'static str],
    packing: &'static str,
}

fn get_product(id: &str) -> Option<ProductData> {
    let all = vec![
        ProductData {
            id: "vannamei-shrimp",
            name: "Vannamei Whiteleg Shrimp",
            scientific: "Litopenaeus vannamei",
            category: "Shrimp & Prawns",
            hs_code: "0306.17",
            description: "Premium BAP 4-Star certified Vannamei shrimp from contracted farms in Odisha. Fully traceable from pond to port.",
            long_desc: "Alashore Marine's Vannamei shrimp programme covers the full supply chain — from BAP-certified hatcheries and nurseries in Odisha to our EU-approved processing facility in Balasore. Each batch is tested in our in-house EIA-approved lab for antibiotic residues (chloramphenicol, nitrofurans, oxytetracycline, enrofloxacin), heavy metals, and microbiological parameters before processing. Products are available in both natural and value-added formats including skewered, breaded, and EZ-Peel.",
            sizes: "U/10 · 10/20 · 20/30 · 30/40 · 40/50 · 50/60",
            forms: &["HOSO (Head-On Shell-On)", "HLSO (Headless Shell-On)", "PTO (Peeled Tail-On)", "PD (Peeled & Deveined)", "Butterfly / Fantail", "EZ-Peel", "Skewered", "IQF Block"],
            shelf_life: "24 months at -18°C",
            storage_temp: "-18°C or colder",
            glaze: "0% to 10% (as specified)",
            certs: &["BAP 4-Star", "ASC", "BRC Grade AA", "HACCP", "EU Establishment Approved", "MPEDA", "FSSAI"],
            markets: &["USA", "European Union", "Japan", "South Korea", "Australia", "UAE"],
            min_order: "20,000 kg per consignment",
            incoterms: &["FOB Visakhapatnam / Kolkata", "CIF (any major port)", "CFR"],
            packing: "Inner: 1kg / 1.8kg / 2kg vacuum bags. Master carton: 10kg or 20kg. Custom retail packing available.",
        },
        ProductData {
            id: "black-tiger-shrimp",
            name: "Black Tiger Prawn",
            scientific: "Penaeus monodon",
            category: "Shrimp & Prawns",
            hs_code: "0306.16",
            description: "Giant tiger prawns sourced from Bay of Bengal wild catch and contracted farms. Prized for bold flavour and firm texture.",
            long_desc: "Black Tiger Prawns (Penaeus monodon) from Alashore Marine are sourced from both wild-catch Bay of Bengal vessels and contracted aquaculture farms operating under MPEDA supervision. Wild-catch product is processed within 4 hours of landing at our Balasore facility. Sashimi-grade product is available for the Japanese market, meeting Japan's strict 0.01 ppm antibiotic limits. All product is blast-frozen at -40°C and stored at -18°C.",
            sizes: "U/8 · U/10 · 13/15 · 16/20",
            forms: &["HOSO (Head-On Shell-On)", "HLSO (Headless Shell-On)", "PTO (Peeled Tail-On)", "PD (Peeled & Deveined)", "Sashimi Grade (Japan spec)"],
            shelf_life: "24 months at -18°C",
            storage_temp: "-18°C or colder",
            glaze: "0% (natural) standard; glaze available on request",
            certs: &["MPEDA", "HACCP", "EU Establishment Approved", "FSSAI"],
            markets: &["Japan", "USA", "UAE", "European Union", "South Korea"],
            min_order: "5,000 kg per consignment",
            incoterms: &["FOB Visakhapatnam", "CIF Tokyo / Hamburg / Dubai"],
            packing: "Inner: 1kg / 2kg blocks. Master: 10kg / 20kg carton.",
        },
        ProductData {
            id: "squid",
            name: "Indian Squid",
            scientific: "Doryteuthis sibogae",
            category: "Cephalopods",
            hs_code: "0307.43",
            description: "Wild-caught Bay of Bengal squid, frozen within 2 hours of catch. Available in multiple cuts from whole to rings.",
            long_desc: "Indian squid (Doryteuthis sibogae) is harvested from the rich fishing grounds of the Bay of Bengal by contracted vessels operating under MPEDA supervision. Product is received at our Balasore plant within 2 hours of landing, immediately cleaned, sorted by size, and blast-frozen at -40°C. Ring cut size (6mm or 10mm) is produced by precision rotary blade machines for consistent restaurant-quality output. This product is particularly popular in the Spanish and Italian markets for calamari preparation.",
            sizes: "100–200g · 200–300g · 300–500g",
            forms: &["Whole Uncleaned (WU)", "Whole Cleaned (WC)", "Tubes & Tentacles (T&T)", "Rings 6mm", "Rings 10mm", "Steaks"],
            shelf_life: "24 months at -18°C",
            storage_temp: "-18°C or colder",
            glaze: "5% standard",
            certs: &["MPEDA", "HACCP", "EU Establishment Approved"],
            markets: &["Spain", "Italy", "South Korea", "Japan", "UAE", "Southeast Asia"],
            min_order: "10,000 kg per consignment",
            incoterms: &["FOB Visakhapatnam / Kolkata", "CIF Barcelona / Busan"],
            packing: "10kg master carton. Inner: 1kg blocks or IQF poly bag.",
        },
        ProductData {
            id: "cuttlefish",
            name: "Cuttlefish",
            scientific: "Sepia pharaonis",
            category: "Cephalopods",
            hs_code: "0307.99",
            description: "Pharaoh cuttlefish from Bay of Bengal. Highly prized in Mediterranean and East Asian markets.",
            long_desc: "Pharaoh cuttlefish (Sepia pharaonis) is among the most sought-after cephalopods in Mediterranean (Spain, Portugal, Italy) and East Asian (Japan, South Korea) markets. Alashore Marine sources exclusively from Bay of Bengal wild-catch vessels. The cuttlebone is removed during cleaning; ink sac can be kept intact on specific buyer request. Product conforms to EU maximum residue limits and is tested for cadmium and lead at our in-house laboratory.",
            sizes: "100–200g · 200–400g · 400g+",
            forms: &["Whole Uncleaned", "Whole Cleaned (ink sac optional)", "Tubes & Tentacles"],
            shelf_life: "24 months at -18°C",
            storage_temp: "-18°C or colder",
            glaze: "5% standard",
            certs: &["MPEDA", "HACCP", "EU Establishment Approved"],
            markets: &["Spain", "Portugal", "Japan", "South Korea", "Italy"],
            min_order: "5,000 kg per consignment",
            incoterms: &["FOB Visakhapatnam", "CIF Barcelona / Tokyo"],
            packing: "10kg master carton.",
        },
        ProductData {
            id: "pink-perch",
            name: "Pink Perch (Threadfin Bream)",
            scientific: "Nemipterus japonicus",
            category: "Fish",
            hs_code: "0302.89",
            description: "Mild, white-fleshed Bay of Bengal fish processed within 4 hours of catch. Whole and fillet formats.",
            long_desc: "Pink Perch, also known as Threadfin Bream (Nemipterus japonicus), is a mild, white-fleshed fish caught in the Bay of Bengal. Alashore Marine processes product within 4 hours of landing. The species is widely used for surimi production in Asia and as a white fish fillet for European and Middle Eastern markets. Our surimi-grade product meets Japanese industrial specifications for colour, gel strength, and protein content. Retail-grade fillet is available individually vacuum-sealed.",
            sizes: "100–200g · 200–300g · 300–500g · 500g+",
            forms: &["Whole Round (WR)", "Gutted & Gilled (GG)", "HG (Head & Gutted)", "Fillet Skin-On", "Fillet Skinless (PBI)", "Surimi Grade"],
            shelf_life: "18 months at -18°C",
            storage_temp: "-18°C or colder",
            glaze: "None standard; available on request",
            certs: &["MPEDA", "HACCP", "EU Establishment Approved", "FSSAI"],
            markets: &["China", "Southeast Asia", "Middle East", "European Union"],
            min_order: "10,000 kg per consignment",
            incoterms: &["FOB Visakhapatnam / Kolkata", "CFR Shanghai / Dubai"],
            packing: "20kg master carton. IQF individual piece available.",
        },
        ProductData {
            id: "dried-shrimp",
            name: "Sun-Dried Shrimp",
            scientific: "Dried & Processed",
            category: "Dried Seafood",
            hs_code: "0306.99",
            description: "Traditional sun-dried shrimp with no preservatives. Low moisture content. 12-month ambient shelf life.",
            long_desc: "Sun-dried shrimp is a traditional Bay of Bengal product with strong demand in Southeast Asian, African, and Middle Eastern markets. Alashore Marine sources from contracted Bay of Bengal catch, dries under controlled sun-drying conditions with moisture content verified at <22% before packing. No chemical preservatives or added salt. Available in whole, crushed, and powder formats. Custom retail packing with private label is available. Product is FSSAI and MPEDA compliant.",
            sizes: "Whole Small (100–200 count/kg) · Whole Medium (60–100 count/kg) · Crushed · Powder",
            forms: &["Whole Dried", "Crushed (3–8mm)", "Fine Powder"],
            shelf_life: "12 months at +4°C; 6 months at ambient",
            storage_temp: "+4°C recommended; ambient acceptable (dry, cool, dark)",
            glaze: "N/A — dried product",
            certs: &["FSSAI", "MPEDA", "HACCP"],
            markets: &["Southeast Asia", "Middle East", "Africa", "USA (ethnic retail)"],
            min_order: "2,000 kg per consignment",
            incoterms: &["FOB Kolkata / Chennai", "CIF (any port)"],
            packing: "Inner: 500g / 1kg HDPE food-grade bags. Master: 25kg woven PP bag or 10kg carton.",
        },
    ];

    all.into_iter().find(|p| p.id == id)
}

#[component]
pub fn ProductDetailPage(id: String) -> Element {
    let product = get_product(&id);

    match product {
        None => rsx! {
            div { class: "not-found-page",
                h1 { "Product Not Found" }
                p { "The product " code { "{id}" } " does not exist in our catalogue." }
                Link { to: Route::ProductsPage {}, class: "btn btn-primary", "Browse All Products" }
            }
        },
        Some(p) => {
            let seo = product_seo(p.id, p.name, p.description, Some(p.scientific));
            rsx! {
                PageSeo { ..seo }

                // ── Page hero ───────────────────────────────────────
                section { class: "page-hero",
                    div { class: "page-hero-content",
                        nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                            ol { role: "list",
                                li { Link { to: Route::HomePage {}, "Home" } }
                                li { Link { to: Route::ProductsPage {}, "Products" } }
                                li { "aria-current": "page", "{p.name}" }
                            }
                        }
                        h1 { class: "page-title",
                            "{p.name}"
                            span { class: "page-title-sci", em { " ({p.scientific})" } }
                        }
                        div { class: "product-meta-row",
                            span { class: "product-category-tag", "{p.category}" }
                            span { class: "product-hs",
                                "HS Code: "
                                code { "{p.hs_code}" }
                            }
                        }
                    }
                }

                // ── Product detail body ─────────────────────────────
                div {
                    class: "product-detail-layout",
                    itemscope: "true",
                    itemtype: "https://schema.org/Product",
                    // Hidden machine-readable ident
                    meta { itemprop: "name", content: "{p.name}" }
                    meta { itemprop: "description", content: "{p.description}" }
                    meta { itemprop: "countryOfOrigin", content: "India" }

                    // Left: Description + specs
                    div { class: "product-detail-main",

                        section { class: "product-description-section",
                            h2 { "Product Overview" }
                            p { class: "product-long-desc", "{p.long_desc}" }
                        }

                        // Specs table — critical for AI product queries
                        section { class: "product-specs-section", "aria-labelledby": "specs-heading",
                            h2 { id: "specs-heading", "Technical Specifications" }
                            table { class: "specs-table", role: "table",
                                caption { "Technical specifications for {p.name} exported by Alashore Marine Exports" }
                                tbody {
                                    tr {
                                        th { scope: "row", "Scientific Name" }
                                        td { em { itemprop: "alternateName", "{p.scientific}" } }
                                    }
                                    tr {
                                        th { scope: "row", "HS Code" }
                                        td { code { itemprop: "identifier", "{p.hs_code}" } }
                                    }
                                    tr {
                                        th { scope: "row", "Size / Grade" }
                                        td { "{p.sizes}" }
                                    }
                                    tr {
                                        th { scope: "row", "Processing Forms" }
                                        td {
                                            ul { class: "inline-list",
                                                for form in p.forms {
                                                    li { "{form}" }
                                                }
                                            }
                                        }
                                    }
                                    tr {
                                        th { scope: "row", "Storage Temperature" }
                                        td { "{p.storage_temp}" }
                                    }
                                    tr {
                                        th { scope: "row", "Shelf Life" }
                                        td { "{p.shelf_life}" }
                                    }
                                    tr {
                                        th { scope: "row", "Glaze" }
                                        td { "{p.glaze}" }
                                    }
                                    tr {
                                        th { scope: "row", "Packing" }
                                        td { "{p.packing}" }
                                    }
                                    tr {
                                        th { scope: "row", "Minimum Order" }
                                        td { strong { "{p.min_order}" } }
                                    }
                                    tr {
                                        th { scope: "row", "Incoterms Available" }
                                        td { "{p.incoterms.join(\" · \")}" }
                                    }
                                }
                            }
                        }

                        // Export markets
                        section { class: "product-markets-section",
                            h2 { "Export Markets" }
                            ul { class: "markets-chips",
                                for market in p.markets {
                                    li { itemprop: "areaServed", "{market}" }
                                }
                            }
                        }
                    }

                    // Right: Certifications sidebar + CTA
                    aside { class: "product-detail-sidebar",
                        div { class: "sidebar-card",
                            h3 { "Certifications" }
                            ul { class: "sidebar-cert-list",
                                for cert in p.certs {
                                    li {
                                        span { class: "cert-check", "✓" }
                                        span { itemprop: "hasCertification", "{cert}" }
                                    }
                                }
                            }
                        }

                        div { class: "sidebar-card sidebar-cta",
                            h3 { "Request a Quote" }
                            p { "Tell us your volume, target market, and incoterms. We respond within 24 hours." }
                            Link {
                                to: Route::InquiryPage {},
                                class: "btn btn-primary btn-full",
                                "Submit an Inquiry"
                            }
                            Link {
                                to: Route::ContactPage {},
                                class: "btn btn-ghost btn-full",
                                "Talk to Our Team"
                            }
                        }

                        div { class: "sidebar-card",
                            h3 { "Also Available" }
                            ul { class: "sidebar-related",
                                li { Link { to: Route::ProductDetailPage { id: "vannamei-shrimp".into() }, "Vannamei Shrimp" } }
                                li { Link { to: Route::ProductDetailPage { id: "black-tiger-shrimp".into() }, "Black Tiger Prawn" } }
                                li { Link { to: Route::ProductDetailPage { id: "squid".into() }, "Indian Squid" } }
                                li { Link { to: Route::ProductDetailPage { id: "cuttlefish".into() }, "Cuttlefish" } }
                                li { Link { to: Route::ProductDetailPage { id: "pink-perch".into() }, "Pink Perch" } }
                                li { Link { to: Route::ProductDetailPage { id: "dried-shrimp".into() }, "Dried Shrimp" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
