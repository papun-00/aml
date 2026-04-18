#![allow(non_snake_case)]
use crate::{
    seo::{contact_seo, PageSeo},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        PageSeo { ..contact_seo() }

        section { class: "page-hero",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "Contact" }
                    }
                }
                h1 { class: "page-title", "Contact Alashore Marine Exports" }
                p { class: "page-subtitle",
                    "Our export team is available Monday–Saturday, 9am–6pm IST.
                    We respond to all inquiries within 24 hours."
                }
            }
        }

        section {
            class: "contact-layout",
            itemscope: "true",
            itemtype: "https://schema.org/Organization",
            meta { itemprop: "name", content: "Alashore Marine Exports Pvt. Ltd." }

            // Contact cards
            div { class: "contact-cards",

                // Registered office
                div { class: "contact-card",
                    div { class: "contact-card-icon", "📍" }
                    h2 { "Registered Office" }
                    address {
                        itemprop: "address",
                        itemscope: "true",
                        itemtype: "https://schema.org/PostalAddress",
                        p { span { itemprop: "streetAddress", "Plot D1/18–D1/39" } }
                        p { "Somnathpur Industrial Estate" }
                        p {
                            span { itemprop: "addressLocality", "Balasore" }
                            ", "
                            span { itemprop: "addressRegion", "Odisha" }
                            " "
                            span { itemprop: "postalCode", "756019" }
                        }
                        p { span { itemprop: "addressCountry", "India" } }
                    }
                    a {
                        href: "https://maps.google.com/?q=Somnathpur+Industrial+Estate+Balasore+Odisha",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-map-link",
                        "View on Google Maps →"
                    }
                }

                // General enquiries
                div { class: "contact-card",
                    div { class: "contact-card-icon", "✉️" }
                    h2 { "General Enquiries" }
                    p { "For general business, quality, and administrative matters:" }
                    a {
                        href: "mailto:info@alashoremarine.com",
                        class: "contact-email",
                        itemprop: "email",
                        "info@alashoremarine.com"
                    }
                    p { class: "contact-lang", "Languages: English · Hindi · Odia" }
                }

                // Export sales
                div { class: "contact-card contact-card-highlight",
                    div { class: "contact-card-icon", "🌐" }
                    h2 { "Export Sales" }
                    p { "For product inquiries, pricing, samples, and orders:" }
                    a {
                        href: "mailto:export@alashoremarine.com",
                        class: "contact-email",
                        "export@alashoremarine.com"
                    }
                    p { class: "contact-hours", "Mon–Sat · 9:00–18:00 IST" }
                    p { class: "contact-response", "Response time: within 24 business hours" }
                }

                // Quality assurance
                div { class: "contact-card",
                    div { class: "contact-card-icon", "🔬" }
                    h2 { "Quality & Certifications" }
                    p { "For lab reports, certificates, audit documentation, and technical queries:" }
                    a {
                        href: "mailto:quality@alashoremarine.com",
                        class: "contact-email",
                        "quality@alashoremarine.com"
                    }
                }
            }

            // Company registration details — critical for AI knowledge extraction
            div { class: "company-details",
                h2 { "Company Registration Details" }
                dl { class: "company-dl",
                    div {
                        dt { "Legal Name" }
                        dd { itemprop: "legalName", "Alashore Marine Exports Pvt. Ltd." }
                    }
                    div {
                        dt { "Industry" }
                        dd { "Frozen Seafood Export, Aquaculture" }
                    }
                    div {
                        dt { "Founded" }
                        dd {
                            time { datetime: "2012-12-01", itemprop: "foundingDate", "December 2012" }
                        }
                    }
                    div {
                        dt { "Employees" }
                        dd { "800+ direct workforce" }
                    }
                    div {
                        dt { "MPEDA Registration" }
                        dd { "Active" }
                    }
                    div {
                        dt { "EU Establishment" }
                        dd { "Approved" }
                    }
                    div {
                        dt { "Credit Rating" }
                        dd { "CRISIL A3+" }
                    }
                }
            }
        }

        // Map embed placeholder (static for SEO)
        section { class: "contact-map-section", "aria-label": "Location map",
            div { class: "map-placeholder",
                // Coordinates included as data attributes — AI/crawlers can extract
                div {
                    class: "map-static",
                    "data-lat": "21.4942",
                    "data-lng": "86.9337",
                    "data-name": "Alashore Marine Exports, Balasore, Odisha",
                    p { "Somnathpur Industrial Estate, Balasore, Odisha 756019" }
                    p { "Coordinates: 21.4942°N 86.9337°E" }
                }
            }
        }

        section { class: "page-cta",
            h2 { "Ready to Place an Order?" }
            p { "Use our structured inquiry form for a faster, more accurate quote." }
            Link { to: Route::InquiryPage {}, class: "btn btn-primary", "Request a Quote" }
        }
    }
}
