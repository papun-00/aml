use frontend::products::{
    all_parsed_products, all_product_sources, get_parsed_product, list_product_slugs, load_product,
    parse_product, render_markdown_to_html,
};
use std::collections::HashSet;

#[test]
fn test_all_product_sources_returns_six() {
    assert_eq!(all_product_sources().len(), 6);
}

#[test]
fn test_list_product_slugs_returns_six() {
    let slugs = list_product_slugs();
    assert_eq!(slugs.len(), 6);
    let unique: HashSet<_> = slugs.iter().collect();
    assert_eq!(unique.len(), 6, "Slugs must be unique");
}

#[test]
fn test_load_product_known_slugs() {
    for slug in list_product_slugs() {
        assert!(load_product(slug).is_some(), "Missing product: {slug}");
    }
}

#[test]
fn test_load_product_unknown_slug() {
    assert!(load_product("nonexistent").is_none());
}

#[test]
fn test_all_parsed_products_returns_six() {
    assert_eq!(all_parsed_products().len(), 6);
}

#[test]
fn test_parsed_products_have_valid_frontmatter() {
    for (fm, _body) in all_parsed_products() {
        assert!(!fm.id.is_empty(), "Product ID is empty");
        assert!(!fm.name.is_empty(), "Product name is empty for {}", fm.id);
        assert!(
            !fm.scientific_name.is_empty(),
            "Scientific name empty for {}",
            fm.id
        );
        assert!(!fm.category.is_empty(), "Category empty for {}", fm.id);
        assert!(!fm.hs_code.is_empty(), "HS code empty for {}", fm.id);
        assert!(!fm.certs.is_empty(), "No certs for {}", fm.id);
        assert!(!fm.markets.is_empty(), "No markets for {}", fm.id);
        assert!(!fm.min_order.is_empty(), "No min_order for {}", fm.id);
        assert!(!fm.short_desc.is_empty(), "No short_desc for {}", fm.id);
    }
}

#[test]
fn test_parsed_products_have_body_content() {
    for (fm, body) in all_parsed_products() {
        assert!(!body.is_empty(), "Product {} has empty body", fm.id);
        assert!(
            body.contains("## Product Overview"),
            "Product {} missing overview section",
            fm.id
        );
        assert!(
            body.contains("## Technical Specifications"),
            "Product {} missing specs section",
            fm.id
        );
    }
}

#[test]
fn test_get_parsed_product_vannamei() {
    let (fm, body) = get_parsed_product("vannamei-shrimp").expect("vannamei-shrimp not found");
    assert_eq!(fm.name, "Vannamei Whiteleg Shrimp");
    assert_eq!(fm.scientific_name, "Litopenaeus vannamei");
    assert_eq!(fm.category, "Shrimp");
    assert!(fm.featured);
    assert!(body.contains("BAP 4-Star"));
}

#[test]
fn test_get_parsed_product_dried_shrimp() {
    let (fm, _body) = get_parsed_product("dried-shrimp").expect("dried-shrimp not found");
    assert_eq!(fm.name, "Sun-Dried Shrimp");
    assert_eq!(fm.category, "Dried");
    assert!(!fm.featured);
}

#[test]
fn test_get_parsed_product_squid_has_tag() {
    let (fm, _body) = get_parsed_product("squid").expect("squid not found");
    assert_eq!(fm.tag, "Popular");
}

#[test]
fn test_parse_product_invalid_input() {
    assert!(parse_product("").is_none());
    assert!(parse_product("no frontmatter here").is_none());
    assert!(parse_product("---\ninvalid toml!!!").is_none());
}

#[test]
fn test_render_markdown_to_html_basic() {
    let html = render_markdown_to_html("## Title\n\nParagraph with **bold**.");
    assert!(html.contains("<h2>"));
    assert!(html.contains("<strong>bold</strong>"));
}

#[test]
fn test_render_markdown_to_html_table() {
    let md = "| Col A | Col B |\n|-------|-------|\n| val1 | val2 |";
    let html = render_markdown_to_html(md);
    assert!(html.contains("<table>"));
    assert!(html.contains("val1"));
}

#[test]
fn test_product_ids_match_slugs() {
    let slugs: HashSet<_> = list_product_slugs().into_iter().collect();
    let parsed = all_parsed_products();
    let ids: HashSet<_> = parsed.iter().map(|(fm, _)| fm.id.as_str()).collect();
    assert_eq!(slugs, ids, "Product slugs must match frontmatter IDs");
}

#[test]
fn test_all_categories_represented() {
    let categories: HashSet<_> = all_parsed_products()
        .iter()
        .map(|(fm, _)| fm.category.clone())
        .collect();
    for expected in &["Shrimp", "Cephalopods", "Fish", "Dried"] {
        assert!(
            categories.contains(*expected),
            "Missing category: {expected}"
        );
    }
}
