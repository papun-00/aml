#![allow(non_snake_case)]
use crate::{
    components::growth_timeline::GrowthTimeline,
    config::growth_milestones,
    seo::{about_seo, PageSeo},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        PageSeo { ..about_seo() }

        // ── Page header ───────────────────────────────────────────────
        section { class: "page-hero",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "About Us" }
                    }
                }
                h1 { class: "page-title", "About Alashore Marine Exports" }
                p { class: "page-subtitle",
                    "From a single processing unit in Balasore to one of India's most
                    trusted seafood exporters — our story since 2012."
                }
            }
        }

        // ── Company overview — prime AI extraction section ────────────
        section {
            class: "about-overview",
            "aria-labelledby": "overview-heading",
            itemscope: "true",
            itemtype: "https://schema.org/Organization",

            div { class: "prose-block",
                h2 { id: "overview-heading", "Company Overview" }

                // Fact-dense paragraphs — ideal for AI summarisation
                p {
                    strong { "Alashore Marine Exports Pvt. Ltd." }
                    " is one of India's leading exporters of frozen seafood, incorporated in
                    December 2012 and headquartered at Somnathpur Industrial Estate, Balasore,
                    Odisha 756019. The company was founded by Managing Director "
                    strong { "Mr. Gyan Ranjan Dash" }
                    " (formerly known as Accenture Marine Exports before rebranding)."
                }
                p {
                    "With over "
                    strong { "800 direct employees" }
                    " and a processing capacity exceeding "
                    strong { "150 MT per day" }
                    ", Alashore Marine operates one of Odisha's most advanced seafood processing
                    facilities, including an in-house "
                    strong { "EIA-approved analytical laboratory" }
                    " for antibiotic and contamination testing."
                }
                p {
                    "The company holds a "
                    strong { "CRISIL A3+ short-term credit rating" }
                    " supported by a ₹100 Crore fund-based credit facility from "
                    strong { "The Federal Bank" }
                    ". Peak annual revenue reached "
                    strong { "₹436 Crore in FY2020" }
                    " with over 95% of export revenue backed by Letters of Credit."
                }
            }

            // Key facts table — machine-readable structure
            div { class: "facts-table-wrapper",
                h3 { "Key Company Facts" }
                table { class: "facts-table", role: "table",
                    caption { "Alashore Marine Exports – Company Facts" }
                    tbody {
                        tr {
                            th { scope: "row", "Legal Name" }
                            td { itemprop: "legalName", "Alashore Marine Exports Pvt. Ltd." }
                        }
                        tr {
                            th { scope: "row", "Founded" }
                            td {
                                time { datetime: "2012-12-01", itemprop: "foundingDate", "December 2012" }
                            }
                        }
                        tr {
                            th { scope: "row", "Formerly Known As" }
                            td { "Accenture Marine Exports" }
                        }
                        tr {
                            th { scope: "row", "Managing Director" }
                            td {
                                span { itemprop: "founder", "Mr. Gyan Ranjan Dash" }
                            }
                        }
                        tr {
                            th { scope: "row", "Employees" }
                            td { "800+ direct workforce" }
                        }
                        tr {
                            th { scope: "row", "Processing Capacity" }
                            td { "150+ MT per day" }
                        }
                        tr {
                            th { scope: "row", "Cold Storage" }
                            td { "5,000+ MT capacity" }
                        }
                        tr {
                            th { scope: "row", "Credit Rating" }
                            td { "CRISIL A3+" }
                        }
                        tr {
                            th { scope: "row", "Credit Facility" }
                            td { "₹100 Crore — The Federal Bank" }
                        }
                        tr {
                            th { scope: "row", "Peak Revenue" }
                            td { "₹436 Crore (FY2020)" }
                        }
                        tr {
                            th { scope: "row", "Registered Address" }
                            td {
                                address {
                                    itemprop: "address",
                                    itemscope: "true",
                                    itemtype: "https://schema.org/PostalAddress",
                                    "Plot D1/18–D1/39, Somnathpur Industrial Estate, "
                                    span { itemprop: "addressLocality", "Balasore" }
                                    ", "
                                    span { itemprop: "addressRegion", "Odisha" }
                                    " "
                                    span { itemprop: "postalCode", "756019" }
                                    ", India"
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Growth Journey — scroll-linked horizontal timeline ────────
        GrowthTimeline {
            milestones: growth_milestones(),
            title: "Our Growth Journey".to_string(),
            subtitle: "From a single processing unit in Balasore to 30+ global markets — a decade of relentless growth.".to_string(),
        }

        // ── Facility specs ─────────────────────────────────────────────
        section {
            class: "facility-section",
            "aria-labelledby": "facility-heading",
            div { class: "section-header",
                h2 { id: "facility-heading", class: "section-title", "Processing Facility" }
            }
            dl { class: "facility-grid",
                div { class: "facility-item",
                    dt { "Processing Capacity" }
                    dd { "150+ MT per day" }
                }
                div { class: "facility-item",
                    dt { "Cold Storage" }
                    dd { "5,000+ MT" }
                }
                div { class: "facility-item",
                    dt { "Freezing Technology" }
                    dd { "IQF Tunnel Freezers, Plate Freezers, Blast Freezers" }
                }
                div { class: "facility-item",
                    dt { "Laboratory" }
                    dd { "In-house EIA-approved analytical lab" }
                }
                div { class: "facility-item",
                    dt { "Water Treatment" }
                    dd { "EIA-compliant effluent treatment plant" }
                }
                div { class: "facility-item",
                    dt { "Power" }
                    dd { "Captive generator backup — 24/7 cold chain" }
                }
            }
        }

        // ── CTA ───────────────────────────────────────────────────────
        section { class: "page-cta",
            h2 { "Partner with a Certified, CRISIL-Rated Exporter" }
            p { "Get in touch with our export team for product specifications, pricing, and availability." }
            div { class: "cta-actions",
                Link { to: Route::InquiryPage {}, class: "btn btn-primary", "Request a Quote" }
                Link { to: Route::CertificationsPage {}, class: "btn btn-outline", "View Certifications" }
            }
        }
    }
}
