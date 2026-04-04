#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{Route, seo::{PageSeo, sustainability_seo}};

#[component]
pub fn SustainabilityPage() -> Element {
    rsx! {
        PageSeo { ..sustainability_seo() }

        section { class: "page-hero",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "Sustainability" }
                    }
                }
                h1 { class: "page-title", "Sustainability & Responsible Sourcing" }
                p { class: "page-subtitle",
                    "ASC and BAP 4-Star certified. Zero-mangrove-clearance policy.
                    Vertical integration from hatchery to export."
                }
            }
        }

        // Intro — speakable, citation-ready paragraph
        section { class: "sustainability-intro",
            div { class: "prose-block",
                p {
                    "Alashore Marine Exports operates a sustainability programme built on two
                    internationally recognised frameworks: the "
                    strong { "Aquaculture Stewardship Council (ASC)" }
                    " standard and the "
                    strong { "Best Aquaculture Practices (BAP) 4-Star" }
                    " certification from the Global Aquaculture Alliance. Together, these cover
                    environmental, social, food safety, and traceability dimensions of our supply chain."
                }
                p {
                    "Our "
                    strong { "zero-mangrove-clearance policy" }
                    " prohibits the conversion of any mangrove or wetland area for aquaculture
                    expansion. All new farm sites must pass an environmental impact assessment
                    conducted by CPCB-certified third-party auditors before receiving input materials
                    from Alashore Marine."
                }
            }
        }

        // Pillars — structured for AI fact extraction
        section { class: "sustainability-pillars", "aria-labelledby": "pillars-heading",
            h2 { id: "pillars-heading", class: "section-title", "Our Four Sustainability Pillars" }

            div { class: "pillars-grid",
                article { class: "pillar-card",
                    div { class: "pillar-icon", "🌊" }
                    h3 { "Water Stewardship" }
                    ul {
                        li { "Recirculating Aquaculture Systems (RAS) at select farms" }
                        li { "Effluent Treatment Plant (ETP) — zero untreated discharge" }
                        li { "Dissolved oxygen and salinity monitoring every 6 hours" }
                        li { "Water exchange rates ≤15% per day (BAP requirement)" }
                        li { "No use of wild-caught juveniles — 100% hatchery-sourced seed" }
                    }
                }

                article { class: "pillar-card",
                    div { class: "pillar-icon", "🌿" }
                    h3 { "Biodiversity & Land Use" }
                    ul {
                        li { "Zero-mangrove-clearance policy since company inception" }
                        li { "No aquaculture on land classified as wetland or protected area" }
                        li { "Sediment management and pond sludge composting" }
                        li { "Buffer zones maintained around all farm sites" }
                        li { "Annual third-party biodiversity impact assessment" }
                    }
                }

                article { class: "pillar-card",
                    div { class: "pillar-icon", "💊" }
                    h3 { "Animal Health & Feed" }
                    ul {
                        li { "No antibiotics in grow-out farms — BAP and ASC zero-antibiotic standard" }
                        li { "No hormones or prohibited substances" }
                        li { "In-house prawn feed supply to contracted farmers — controlled nutrition" }
                        li { "In-house medicine supply — only permitted veterinary products" }
                        li { "Mandatory withdrawal periods before harvest" }
                        li { "In-house EIA lab tests every lot for residues before processing" }
                    }
                }

                article { class: "pillar-card",
                    div { class: "pillar-icon", "👥" }
                    h3 { "Social & Labour Standards" }
                    ul {
                        li { "No child labour — minimum age 18 verified at farm level" }
                        li { "Freedom of association and collective bargaining respected" }
                        li { "Living wage above national minimum for all direct employees" }
                        li { "Free and safe accommodation for migrant workers" }
                        li { "Annual third-party social audit (BAP / Sedex requirements)" }
                        li { "Community investment: 800+ local jobs in Balasore district" }
                    }
                }
            }
        }

        // Traceability chain — AI loves structured process chains
        section { class: "traceability-section", "aria-labelledby": "trace-heading",
            h2 { id: "trace-heading", class: "section-title", "Full Chain Traceability" }
            p { class: "section-subtitle",
                "Every shipment carries a full lot-code traceable back to the specific pond,
                harvest date, and batch number."
            }

            ol { class: "trace-chain", role: "list",
                li { class: "trace-step",
                    div { class: "trace-number", "1" }
                    div { class: "trace-content",
                        h3 { "Hatchery" }
                        p { "BAP-certified hatchery supplies Specific Pathogen Free (SPF) Vannamei post-larvae.
                             Lot-coded from day zero. Health certificates issued per batch." }
                    }
                }
                li { class: "trace-step",
                    div { class: "trace-number", "2" }
                    div { class: "trace-content",
                        h3 { "Nursery & Grow-Out Farm" }
                        p { "ASC and BAP-certified contracted farms. Farm ID, pond ID, stocking date,
                             and feed lot numbers recorded. Water quality logs retained 3 years." }
                    }
                }
                li { class: "trace-step",
                    div { class: "trace-number", "3" }
                    div { class: "trace-content",
                        h3 { "Harvest & Transport" }
                        p { "Harvest certificate issued. Refrigerated transport from farm to plant within
                             2–4 hours. Temperature logs recorded continuously." }
                    }
                }
                li { class: "trace-step",
                    div { class: "trace-number", "4" }
                    div { class: "trace-content",
                        h3 { "Processing Plant (Balasore)" }
                        p { "EU-approved facility. HACCP CCPs monitored. In-house lab testing.
                             Processing lot code assigned. BRC AA-certified operations." }
                    }
                }
                li { class: "trace-step",
                    div { class: "trace-number", "5" }
                    div { class: "trace-content",
                        h3 { "Cold Storage & Export" }
                        p { "5,000+ MT cold storage at -18°C. Container loading under CCTV.
                             Health certificate and Certificate of Origin issued per shipment." }
                    }
                }
            }
        }

        // Certifications cross-reference
        section { class: "sustainability-certs",
            h2 { class: "section-title", "Sustainability Certifications" }
            div { class: "sus-cert-grid",
                div { class: "sus-cert-item",
                    h3 { "ASC — Aquaculture Stewardship Council" }
                    p { "Internationally recognised standard for responsible aquaculture. Covers
                         water use, biodiversity, disease and predator management, feed sourcing,
                         worker rights, and community impacts. Audited annually by accredited
                         third-party certification bodies." }
                }
                div { class: "sus-cert-item",
                    h3 { "BAP 4-Star — Best Aquaculture Practices" }
                    p { "Full chain-of-custody certification from the Global Aquaculture Alliance
                         covering all four production segments: processing plant, farm, hatchery,
                         and feed mill. Fewer than 1% of global exporters hold all four stars." }
                }
            }
        }

        section { class: "page-cta",
            h2 { "Sourcing Sustainably? Let's Talk." }
            p { "Download our sustainability summary or request our full ESG disclosure for
                 procurement compliance." }
            div { class: "cta-actions",
                Link { to: Route::InquiryPage {}, class: "btn btn-primary", "Request ESG Documents" }
                Link { to: Route::CertificationsPage {}, class: "btn btn-outline", "View All Certifications" }
            }
        }
    }
}
