use frontend::config::*;
use std::collections::HashSet;

#[test]
fn test_all_products_returns_six() {
    assert_eq!(all_products().len(), 6);
}

#[test]
fn test_product_ids_are_unique() {
    let products = all_products();
    let ids: HashSet<&str> = products.iter().map(|p| p.id).collect();
    assert_eq!(ids.len(), products.len(), "Duplicate product IDs found");
}

#[test]
fn test_all_products_have_certs() {
    for p in all_products() {
        assert!(!p.certs.is_empty(), "Product '{}' has no certifications", p.name);
    }
}

#[test]
fn test_company_stats_returns_six() {
    assert_eq!(company_stats().len(), 6);
}

#[test]
fn test_certifications_returns_five() {
    assert_eq!(certifications().len(), 5);
}

#[test]
fn test_faqs_returns_four() {
    assert_eq!(faqs().len(), 4);
}

#[test]
fn test_inquiry_products_matches_all_products() {
    let product_ids: HashSet<&str> = all_products().iter().map(|p| p.id).collect();
    let inquiry_ids: HashSet<&str> = inquiry_products().iter().map(|(id, _)| *id).collect();
    assert_eq!(product_ids, inquiry_ids, "Inquiry product IDs must match catalog product IDs");
}

#[test]
fn test_product_categories_cover_all() {
    let products = all_products();
    let categories: HashSet<String> = products.iter().map(|p| p.category.as_str().to_string()).collect();
    for expected in &["Shrimp", "Cephalopods", "Fish", "Dried"] {
        assert!(categories.contains(*expected), "Missing category: {}", expected);
    }
}
