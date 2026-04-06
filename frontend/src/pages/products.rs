#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::config::{all_products, ProductCategory};
use crate::components::product_card::ProductCard;
use crate::{Route, seo::{PageSeo, products_seo}};

#[derive(Clone, PartialEq)]
enum Filter { All, Shrimp, Fish, Cephalopods, Dried }

impl Filter {
    fn matches(&self, category: &ProductCategory) -> bool {
        match self {
            Filter::All => true,
            Filter::Shrimp => *category == ProductCategory::Shrimp,
            Filter::Fish => *category == ProductCategory::Fish,
            Filter::Cephalopods => *category == ProductCategory::Cephalopods,
            Filter::Dried => *category == ProductCategory::Dried,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Filter::All => "All Products",
            Filter::Shrimp => "Shrimp & Prawns",
            Filter::Fish => "Fish",
            Filter::Cephalopods => "Cephalopods",
            Filter::Dried => "Dried",
        }
    }
}

const FILTERS: &[Filter] = &[Filter::All, Filter::Shrimp, Filter::Fish, Filter::Cephalopods, Filter::Dried];

#[component]
pub fn ProductsPage() -> Element {
    let mut active = use_signal(|| Filter::All);
    let products = all_products();

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
                for filter in FILTERS {
                    button {
                        class: if active() == *filter { "filter-tab active" } else { "filter-tab" },
                        role: "tab",
                        "aria-selected": "{active() == *filter}",
                        onclick: {
                            let f = filter.clone();
                            move |_| active.set(f.clone())
                        },
                        "{filter.label()}"
                    }
                }
            }

            // Product grid — reuses the same ProductCard component as the home page
            div {
                class: "products-grid",
                role: "tabpanel",
                "aria-live": "polite",
                itemscope: "true",
                itemtype: "https://schema.org/ItemList",

                for product in products.into_iter().filter(|p| active().matches(&p.category)) {
                    ProductCard {
                        key: "{product.id}",
                        id: product.id.clone(),
                        name: product.name.clone(),
                        scientific_name: product.scientific_name.clone(),
                        description: product.short_desc.clone(),
                        certifications: product.certs.clone(),
                        css_class: product.css_class.clone(),
                        image_url: Some(product.image_url.clone()),
                        featured: product.featured,
                        tag: product.tag.clone(),
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
