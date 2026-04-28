#![allow(non_snake_case)]
use crate::utils::asset_url;
use crate::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProductCardProps {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
    pub description: String,
    pub certifications: Vec<String>,
    pub css_class: String,
    #[props(default)]
    pub image_url: Option<String>,
    #[props(default = false)]
    pub featured: bool,
    #[props(default)]
    pub tag: Option<String>,
}

#[component]
pub fn ProductCard(props: ProductCardProps) -> Element {
    let card_class = if props.featured {
        "product-card featured"
    } else {
        "product-card"
    };

    rsx! {
        article {
            class: "{card_class}",
            itemscope: "true",
            itemtype: "https://schema.org/Product",

            div {
                class: "product-card-visual {props.css_class}",
                if let Some(ref url) = props.image_url {
                    img {
                        src: "{asset_url(url)}",
                        alt: "{props.name}",
                        class: "product-card-img",
                        loading: "lazy",
                        width: "400",
                        height: "300",
                    }
                }
            }

            div {
                class: "product-card-body",

                if let Some(ref tag) = props.tag {
                    div {
                        class: "product-card-tag",
                        "{tag}"
                    }
                }

                h3 {
                    class: "product-card-name",
                    itemprop: "name",
                    "{props.name}"
                }

                p {
                    class: "product-card-sci",
                    em {
                        itemprop: "alternateName",
                        "{props.scientific_name}"
                    }
                }

                p {
                    class: "product-card-desc",
                    itemprop: "description",
                    "{props.description}"
                }

                div {
                    class: "product-card-certs",
                    for cert in props.certifications.iter() {
                        span {
                            key: "{cert}",
                            class: "product-card-cert",
                            "{cert}"
                        }
                    }
                }

                Link {
                    to: Route::ProductDetailPage { id: props.id.clone() },
                    class: "product-card-link",
                    "View Full Specs \u{2192}"
                }
            }
        }
    }
}
