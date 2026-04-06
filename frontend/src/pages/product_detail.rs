#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::products::{get_parsed_product, render_markdown_to_html, all_parsed_products};
use crate::{Route, seo::{PageSeo, product_seo}};

#[component]
pub fn ProductDetailPage(id: String) -> Element {
    let parsed = get_parsed_product(&id);

    match parsed {
        None => rsx! {
            div { class: "not-found-page",
                h1 { "Product Not Found" }
                p { "The product " code { "{id}" } " does not exist in our catalogue." }
                Link { to: Route::ProductsPage {}, class: "btn btn-primary", "Browse All Products" }
            }
        },
        Some((fm, body)) => {
            let seo = product_seo(&fm.id, &fm.name, &fm.short_desc, Some(&fm.scientific_name));
            let html_content = render_markdown_to_html(&body);

            // Related products (exclude current)
            let other_products: Vec<_> = all_parsed_products()
                .into_iter()
                .filter(|(f, _)| f.id != fm.id)
                .collect();

            rsx! {
                PageSeo { ..seo }

                // ── Page hero ─────────────────────────────────────
                section { class: "page-hero",
                    div { class: "page-hero-content",
                        nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                            ol { role: "list",
                                li { Link { to: Route::HomePage {}, "Home" } }
                                li { Link { to: Route::ProductsPage {}, "Products" } }
                                li { "aria-current": "page", "{fm.name}" }
                            }
                        }
                        h1 { class: "page-title",
                            "{fm.name}"
                        }
                        p { class: "page-title-sci",
                            em { "{fm.scientific_name}" }
                        }
                        div { class: "product-meta-row",
                            span { class: "product-category-tag", "{fm.category}" }
                            span { class: "product-hs",
                                "HS Code: "
                                code { "{fm.hs_code}" }
                            }
                        }
                    }
                }

                // ── Product detail body ───────────────────────────
                div {
                    class: "product-detail-layout",
                    itemscope: "true",
                    itemtype: "https://schema.org/Product",
                    meta { itemprop: "name", content: "{fm.name}" }
                    meta { itemprop: "description", content: "{fm.short_desc}" }
                    meta { itemprop: "countryOfOrigin", content: "India" }

                    // Left: rendered markdown content
                    div { class: "product-detail-main",
                        div {
                            class: "product-markdown-content",
                            dangerous_inner_html: "{html_content}"
                        }
                    }

                    // Right sidebar
                    aside { class: "product-detail-sidebar",
                        div { class: "sidebar-card",
                            h3 { "Certifications" }
                            ul { class: "sidebar-cert-list",
                                for cert in &fm.certs {
                                    li {
                                        span { class: "cert-check", "\u{2713}" }
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
                                for (other_fm, _) in &other_products {
                                    li {
                                        Link {
                                            to: Route::ProductDetailPage { id: other_fm.id.clone() },
                                            "{other_fm.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
