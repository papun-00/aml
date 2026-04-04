#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{Route, seo::{PageSeo, about_seo}};

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

        // ── Timeline ──────────────────────────────────────────────────
        section {
            class: "timeline-section",
            "aria-labelledby": "timeline-heading",
            h2 { id: "timeline-heading", class: "section-title", "Our Journey" }
            ol { class: "timeline", role: "list",
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2012", "2012" }
                    div { class: "timeline-content",
                        h3 { "Incorporated as Accenture Marine Exports" }
                        p { "Founded December 2012 in Balasore, Odisha by Mr. Gyan Ranjan Dash.
                             First processing unit established at Somnathpur Industrial Estate." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2014", "2014" }
                    div { class: "timeline-content",
                        h3 { "MPEDA Registration & First EU Exports" }
                        p { "Secured MPEDA registration. Obtained EU Establishment Approval.
                             First shipments to European buyers in Spain and Italy." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2016", "2016" }
                    div { class: "timeline-content",
                        h3 { "BAP Certification & US Market Entry" }
                        p { "Achieved BAP 4-Star certification. Commenced exports to the United States.
                             Expanded processing capacity to 80 MT/day." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2018", "2018" }
                    div { class: "timeline-content",
                        h3 { "BRC AA & ASC Certification" }
                        p { "Achieved BRC Grade AA for global food safety. Obtained ASC certification
                             for aquaculture sustainability. Opened in-house EIA-approved lab." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2019", "2019" }
                    div { class: "timeline-content",
                        h3 { "Rebranded to Alashore Marine Exports" }
                        p { "Company rebranded from Accenture Marine Exports to Alashore Marine Exports
                             Pvt. Ltd. ₹100 Crore Federal Bank credit facility secured." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2020", "2020" }
                    div { class: "timeline-content",
                        h3 { "Peak Revenue — ₹436 Crore" }
                        p { "Record turnover of ₹436 Crore. Workforce exceeded 800. Processing
                             capacity scaled to 150 MT/day. CRISIL A3+ rating awarded." }
                    }
                }
                li { class: "timeline-item",
                    time { class: "timeline-year", datetime: "2024", "2024" }
                    div { class: "timeline-content",
                        h3 { "Continued Growth — 30+ Export Markets" }
                        p { "Exporting to 30+ countries. Ongoing vertical integration investments.
                             New website and digital infrastructure launched." }
                    }
                }
            }
        }

        // ── Facility specs ─────────────────────────────────────────────
        section {
            class: "facility-section",
            "aria-labelledby": "facility-heading",
            h2 { id: "facility-heading", class: "section-title", "Processing Facility" }
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
