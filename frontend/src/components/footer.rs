#![allow(non_snake_case)]
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "site-footer",
            role: "contentinfo",
            // Schema.org microdata for footer — boosts AI extraction of contact info
            itemscope: "true",
            itemtype: "https://schema.org/Organization",

            div { class: "footer-grid",
                // Col 1: Brand
                div { class: "footer-brand",
                    Link { to: Route::HomePage {}, class: "footer-logo", itemprop: "url",
                        div { class: "logo-mark-sm", "AM" }
                        div {
                            span { class: "logo-primary", itemprop: "name", "Alashore Marine Exports" }
                            span { class: "logo-tagline", "Pvt. Ltd." }
                        }
                    }
                    p { class: "footer-tagline",
                        "Premium frozen seafood exporter from Balasore, Odisha — supplying to 30+ countries since 2012."
                    }
                    // Certifications row — visible text for AI crawlers
                    div { class: "footer-certs",
                        span { class: "cert-pill", "BAP 4★" }
                        span { class: "cert-pill", "BRC AA" }
                        span { class: "cert-pill", "ASC" }
                        span { class: "cert-pill", "HACCP" }
                        span { class: "cert-pill", "EU Approved" }
                    }
                }

                // Col 2: Products
                div { class: "footer-col",
                    h3 { class: "footer-heading", "Products" }
                    nav { "aria-label": "Product links",
                        ul { role: "list",
                            li { Link { to: Route::ProductDetailPage { id: "vannamei-shrimp".into() }, "Vannamei Shrimp" } }
                            li { Link { to: Route::ProductDetailPage { id: "black-tiger-shrimp".into() }, "Black Tiger Prawn" } }
                            li { Link { to: Route::ProductDetailPage { id: "squid".into() }, "Indian Squid" } }
                            li { Link { to: Route::ProductDetailPage { id: "cuttlefish".into() }, "Cuttlefish" } }
                            li { Link { to: Route::ProductDetailPage { id: "pink-perch".into() }, "Pink Perch" } }
                            li { Link { to: Route::ProductDetailPage { id: "dried-shrimp".into() }, "Dried Shrimp" } }
                        }
                    }
                }

                // Col 3: Company
                div { class: "footer-col",
                    h3 { class: "footer-heading", "Company" }
                    nav { "aria-label": "Company links",
                        ul { role: "list",
                            li { Link { to: Route::AboutPage {}, "About Us" } }
                            li { Link { to: Route::CertificationsPage {}, "Certifications" } }
                            li { Link { to: Route::SustainabilityPage {}, "Sustainability" } }
                            li { Link { to: Route::ContactPage {}, "Contact" } }
                            li { Link { to: Route::InquiryPage {}, "Request a Quote" } }
                        }
                    }
                }

                // Col 4: Contact — schema.org PostalAddress + ContactPoint
                div { class: "footer-col",
                    h3 { class: "footer-heading", "Contact" }
                    address {
                        class: "footer-address",
                        itemprop: "address",
                        itemscope: "true",
                        itemtype: "https://schema.org/PostalAddress",
                        p {
                            span { itemprop: "streetAddress", "Plot D1/18–D1/39, Somnathpur Industrial Estate" }
                        }
                        p {
                            span { itemprop: "addressLocality", "Balasore" }
                            ", "
                            span { itemprop: "addressRegion", "Odisha" }
                            " "
                            span { itemprop: "postalCode", "756019" }
                        }
                        p { span { itemprop: "addressCountry", "India" } }
                    }
                    div { class: "footer-contact-links",
                        a {
                            href: "mailto:info@alashoremarine.com",
                            itemprop: "email",
                            class: "footer-email",
                            "info@alashoremarine.com"
                        }
                        a {
                            href: "mailto:export@alashoremarine.com",
                            class: "footer-email",
                            "export@alashoremarine.com"
                        }
                    }
                }
            }

            div { class: "footer-bottom",
                p { class: "footer-legal",
                    "© 2024 Alashore Marine Exports Pvt. Ltd. All rights reserved."
                }
                p { class: "footer-credits",
                    "MPEDA Registered · EU Approved Establishment · CRISIL A3+"
                }
            }
        }
    }
}
