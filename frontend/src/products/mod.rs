//! Product content module — compile-time embedded markdown with TOML frontmatter.
//!
//! Each product is a markdown file in `frontend/content/products/` embedded via `include_str!`.

mod parser;

pub use parser::{ProductFrontmatter, parse_product, render_markdown_to_html};

// Compile-time embedded product markdown
const PRODUCT_VANNAMEI_SHRIMP: &str = include_str!("../../content/products/vannamei-shrimp.md");
const PRODUCT_BLACK_TIGER_SHRIMP: &str = include_str!("../../content/products/black-tiger-shrimp.md");
const PRODUCT_SQUID: &str = include_str!("../../content/products/squid.md");
const PRODUCT_CUTTLEFISH: &str = include_str!("../../content/products/cuttlefish.md");
const PRODUCT_PINK_PERCH: &str = include_str!("../../content/products/pink-perch.md");
const PRODUCT_DRIED_SHRIMP: &str = include_str!("../../content/products/dried-shrimp.md");

/// All embedded product markdown sources, in display order.
pub fn all_product_sources() -> Vec<&'static str> {
    vec![
        PRODUCT_VANNAMEI_SHRIMP,
        PRODUCT_BLACK_TIGER_SHRIMP,
        PRODUCT_PINK_PERCH,
        PRODUCT_SQUID,
        PRODUCT_CUTTLEFISH,
        PRODUCT_DRIED_SHRIMP,
    ]
}

/// Load a single product's raw markdown by slug.
pub fn load_product(slug: &str) -> Option<&'static str> {
    match slug {
        "vannamei-shrimp" => Some(PRODUCT_VANNAMEI_SHRIMP),
        "black-tiger-shrimp" => Some(PRODUCT_BLACK_TIGER_SHRIMP),
        "squid" => Some(PRODUCT_SQUID),
        "cuttlefish" => Some(PRODUCT_CUTTLEFISH),
        "pink-perch" => Some(PRODUCT_PINK_PERCH),
        "dried-shrimp" => Some(PRODUCT_DRIED_SHRIMP),
        _ => None,
    }
}

/// List all available product slugs.
pub fn list_product_slugs() -> Vec<&'static str> {
    vec![
        "vannamei-shrimp",
        "black-tiger-shrimp",
        "pink-perch",
        "squid",
        "cuttlefish",
        "dried-shrimp",
    ]
}

/// Parse all products, returning (frontmatter, markdown_body) tuples.
pub fn all_parsed_products() -> Vec<(ProductFrontmatter, String)> {
    all_product_sources()
        .into_iter()
        .filter_map(parse_product)
        .collect()
}

/// Get a single parsed product by slug.
pub fn get_parsed_product(slug: &str) -> Option<(ProductFrontmatter, String)> {
    load_product(slug).and_then(parse_product)
}
