#![allow(non_snake_case)]
use crate::{
    seo::{certifications_seo, PageSeo},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn CertificationsPage() -> Element {
    rsx! {
        PageSeo { ..certifications_seo() }

        section { class: "page-hero",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "Certifications" }
                    }
                }
                h1 { class: "page-title", "Certifications & Quality Standards" }
                p { class: "page-subtitle",
                    "Alashore Marine Exports holds industry's highest tier certifications,
                    independently audited and renewed annually."
                }
            }
        }

        // Summary table — ideal for AI extraction as a reference table
        section { class: "certs-summary", "aria-labelledby": "certs-table-heading",
            h2 { id: "certs-table-heading", class: "sr-only", "Certification Summary Table" }
            table { class: "certs-table", role: "table",
                caption { "Alashore Marine Exports — Certifications held as of 2024" }
                thead {
                    tr {
                        th { scope: "col", "Certification" }
                        th { scope: "col", "Issuing Body" }
                        th { scope: "col", "Scope" }
                        th { scope: "col", "Status" }
                    }
                }
                tbody {
                    tr {
                        td { strong { "BAP 4-Star" } }
                        td { "Global Aquaculture Alliance (GAA)" }
                        td { "Farm, hatchery, feed mill, processing plant" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "BRC Grade AA" } }
                        td { "British Retail Consortium (BRCGS)" }
                        td { "Processing plant — Global Food Safety Standard" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "ASC" } }
                        td { "Aquaculture Stewardship Council" }
                        td { "Farm and supply chain sustainability" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "HACCP" } }
                        td { "Codex Alimentarius / EU" }
                        td { "Hazard Analysis and Critical Control Points" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "EU Establishment Approval" } }
                        td { "European Commission (EC 853/2004)" }
                        td { "Direct export to all 27 EU member states" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "MPEDA" } }
                        td { "Marine Products Export Development Authority" }
                        td { "Indian seafood export authorisation" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "FSSAI" } }
                        td { "Food Safety and Standards Authority of India" }
                        td { "Domestic food safety compliance" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                    tr {
                        td { strong { "CRISIL A3+" } }
                        td { "CRISIL (S&P Global)" }
                        td { "Short-term credit rating — strong repayment capability" }
                        td { span { class: "cert-status active", "Active" } }
                    }
                }
            }
        }

        // Detailed cert cards
        section { class: "certs-detail", "aria-labelledby": "certs-detail-heading",
            div { class: "section-header",
                h2 { id: "certs-detail-heading", class: "section-title", "Certification Details" }
            }

            div { class: "certs-grid",
                CertCard {
                    name: "BAP 4-Star",
                    issuer: "Global Aquaculture Alliance",
                    tier: "Gold",
                    description: "The Best Aquaculture Practices 4-Star is the highest possible BAP chain-of-custody certification. It certifies our entire supply chain: shrimp hatchery, aquafeed mill, grow-out farms, and processing plant — the only certification that covers all four segments. Fewer than 1% of global seafood exporters hold all four BAP stars simultaneously.",
                    what_it_means: "Buyers in the USA, EU, Japan, and Australia recognise BAP 4-Star as the gold standard for aquaculture transparency and traceability.",
                }
                CertCard {
                    name: "BRC Grade AA",
                    issuer: "BRCGS (British Retail Consortium)",
                    tier: "Highest",
                    description: "BRC Grade AA is the highest possible score in the BRCGS Global Standard for Food Safety. Our processing plant achieved AA on unannounced audit — the strictest audit mode. Required for supply to UK and European retailers including Tesco, Sainsbury's, Waitrose, Carrefour, and Metro.",
                    what_it_means: "AA grade unlocks access to premium European and North American retail buyers who mandate BRCGS certification.",
                }
                CertCard {
                    name: "ASC Certified",
                    issuer: "Aquaculture Stewardship Council",
                    tier: "Gold",
                    description: "ASC certification verifies that our aquaculture operations meet environmental and social standards covering water use, biodiversity, disease management, worker rights, and community relations. ASC is recognised by WWF and global sustainability bodies.",
                    what_it_means: "Required for supply to sustainability-committed buyers and retailers with ESG procurement policies.",
                }
                CertCard {
                    name: "HACCP",
                    issuer: "Codex Alimentarius Commission",
                    tier: "Standard",
                    description: "Hazard Analysis and Critical Control Points (HACCP) is the globally mandated food safety management system. Our HACCP plan covers physical, chemical, and microbiological hazards at every Critical Control Point (CCP) — from raw material receipt through freezing, glazing, and dispatch.",
                    what_it_means: "Legally required for export to all major markets. Forms the foundation of BRC, EU, and US FDA compliance.",
                }
                CertCard {
                    name: "EU Establishment Approval",
                    issuer: "European Commission",
                    tier: "Regulatory",
                    description: "Granted under EC Regulation 853/2004 (food of animal origin). EU Establishment Approval allows direct export of our products to all 27 EU member states without additional border inspection. Compliance with EC 852/2004 (food hygiene), EC 1379/2013 (labelling), and EU antibiotic limits is independently verified.",
                    what_it_means: "Non-EU approved establishments cannot legally supply EU buyers. This approval is mandatory for European market access.",
                }
                CertCard {
                    name: "CRISIL A3+",
                    issuer: "CRISIL (S&P Global India)",
                    tier: "Financial",
                    description: "A3+ is a short-term credit rating indicating 'strong' repayment capability. Alashore Marine Exports holds this rating on its ₹100 Crore fund-based credit facility from The Federal Bank. The rating is supported by a current ratio of 2.16x, healthy debtor days, and consistent export performance.",
                    what_it_means: "Demonstrates financial stability, de-risking buyer relationships and enabling favourable LC terms.",
                }
            }
        }

        section { class: "page-cta",
            h2 { "Request Certification Documents" }
            p { "We provide copies of current certificates, audit reports, and lab results upon request
                 under NDA. Contact our quality assurance team." }
            div { class: "cta-actions",
                Link { to: Route::InquiryPage {}, class: "btn btn-primary", "Request Documents" }
                Link { to: Route::ContactPage {}, class: "btn btn-outline", "Contact QA Team" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CertCardProps {
    name: &'static str,
    issuer: &'static str,
    tier: &'static str,
    description: &'static str,
    what_it_means: &'static str,
}

#[component]
fn CertCard(props: CertCardProps) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        article {
            class: "cert-card",
            itemscope: "true",
            itemtype: "https://schema.org/EducationalOccupationalCredential",

            button {
                class: "cert-card-header",
                "aria-expanded": "{open()}",
                onclick: move |_| open.set(!open()),

                div { class: "cert-card-title-row",
                    h3 { class: "cert-card-name", itemprop: "credentialCategory", "{props.name}" }
                    span { class: "cert-tier tier-{props.tier.to_lowercase()}", "{props.tier}" }
                }
                p { class: "cert-card-issuer", itemprop: "recognizedBy", "{props.issuer}" }
                span { class: "cert-card-toggle", if open() { "−" } else { "+" } }
            }

            if open() {
                div { class: "cert-card-body",
                    p { itemprop: "description", "{props.description}" }
                    div { class: "cert-card-impact",
                        strong { "What this means for buyers: " }
                        "{props.what_it_means}"
                    }
                }
            }
        }
    }
}
