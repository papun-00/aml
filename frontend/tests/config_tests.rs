use frontend::config::*;
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Growth Milestones tests
// ---------------------------------------------------------------------------

#[test]
fn test_growth_milestones_returns_seven() {
    assert_eq!(growth_milestones().len(), 7);
}

#[test]
fn test_growth_milestones_years_are_chronological() {
    let milestones = growth_milestones();
    for window in milestones.windows(2) {
        assert!(
            window[0].year <= window[1].year,
            "Milestones not in chronological order: {} > {}",
            window[0].year,
            window[1].year,
        );
    }
}

#[test]
fn test_growth_milestones_have_required_fields() {
    for m in growth_milestones() {
        assert!(m.year >= 2012, "Year {} is before founding", m.year);
        assert!(!m.title.is_empty(), "Milestone {} has empty title", m.year);
        assert!(
            !m.description.is_empty(),
            "Milestone {} has empty description",
            m.year
        );
        assert!(!m.icon.is_empty(), "Milestone {} has empty icon", m.year);
    }
}

#[test]
fn test_growth_milestones_years_are_unique() {
    let milestones = growth_milestones();
    let years: HashSet<u16> = milestones.iter().map(|m| m.year).collect();
    assert_eq!(
        years.len(),
        milestones.len(),
        "Duplicate milestone years found"
    );
}

#[test]
fn test_growth_milestones_has_one_highlight() {
    let highlights: Vec<_> = growth_milestones()
        .into_iter()
        .filter(|m| m.highlight)
        .collect();
    assert!(
        !highlights.is_empty(),
        "At least one milestone should be highlighted"
    );
}

#[test]
fn test_growth_milestones_icons_are_valid() {
    let valid_icons = [
        "anchor",
        "globe",
        "star",
        "shield",
        "refresh",
        "trending-up",
    ];
    for m in growth_milestones() {
        assert!(
            valid_icons.contains(&m.icon),
            "Milestone {} has invalid icon: {}",
            m.year,
            m.icon,
        );
    }
}

#[test]
fn test_all_products_returns_six() {
    assert_eq!(all_products().len(), 6);
}

#[test]
fn test_product_ids_are_unique() {
    let products = all_products();
    let ids: HashSet<&str> = products.iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids.len(), products.len(), "Duplicate product IDs found");
}

#[test]
fn test_all_products_have_certs() {
    for p in all_products() {
        assert!(
            !p.certs.is_empty(),
            "Product '{}' has no certifications",
            p.name
        );
    }
}

#[test]
fn test_all_products_have_markets() {
    for p in all_products() {
        assert!(!p.markets.is_empty(), "Product '{}' has no markets", p.name);
    }
}

#[test]
fn test_all_products_have_min_order() {
    for p in all_products() {
        assert!(
            !p.min_order.is_empty(),
            "Product '{}' has no min_order",
            p.name
        );
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
fn test_certifications_have_unique_names() {
    let certs = certifications();
    let names: HashSet<String> = certs.iter().map(|c| c.name.clone()).collect();
    assert_eq!(
        names.len(),
        certs.len(),
        "Certification names must be unique"
    );
}

#[test]
fn test_certifications_have_unique_ids() {
    let certs = certifications();
    let ids: HashSet<String> = certs.iter().map(|c| c.id.clone()).collect();
    assert_eq!(ids.len(), certs.len(), "Certification ids must be unique");
}

#[test]
fn test_certifications_ids_are_lowercase_alpha() {
    for cert in certifications() {
        assert!(
            cert.id.chars().all(|c| c.is_ascii_lowercase()),
            "id '{}' for '{}' must be lowercase alpha only",
            cert.id,
            cert.name,
        );
    }
}

#[test]
fn test_certifications_brand_colors_are_valid_hex() {
    for cert in certifications() {
        assert!(
            cert.brand_color.starts_with('#') && cert.brand_color.len() == 7,
            "brand_color '{}' for '{}' must be a 7-char hex color (#RRGGBB)",
            cert.brand_color,
            cert.name,
        );
    }
}

#[test]
fn test_certifications_expected_names_present() {
    let names: Vec<String> = certifications().iter().map(|c| c.name.clone()).collect();
    for expected in &["BAP 4-Star", "BRC AA", "ASC", "HACCP", "EU Approved"] {
        assert!(
            names.iter().any(|n| n == expected),
            "Missing certification: {expected}"
        );
    }
}

#[test]
fn test_certifications_all_have_label_short() {
    for cert in certifications() {
        assert!(
            !cert.label_short.is_empty(),
            "Certification '{}' has empty label_short",
            cert.name
        );
    }
}

// ── TOML config-driven layout tests ──────────────────────────
#[test]
fn test_cert_config_loads_successfully() {
    let config = cert_config();
    assert!(!config.certs.is_empty());
}

#[test]
fn test_cert_layout_valid_direction() {
    let layout = cert_layout();
    assert!(
        layout.direction == "horizontal" || layout.direction == "vertical",
        "direction must be 'horizontal' or 'vertical', got '{}'",
        layout.direction,
    );
}

#[test]
fn test_cert_layout_stamp_size_reasonable() {
    let layout = cert_layout();
    assert!(
        layout.stamp_size >= 16 && layout.stamp_size <= 128,
        "stamp_size {} is out of reasonable range 16..128",
        layout.stamp_size
    );
}

#[test]
fn test_cert_layout_gap_reasonable() {
    let layout = cert_layout();
    assert!(
        layout.gap >= 4 && layout.gap <= 64,
        "gap {} is out of reasonable range 4..64",
        layout.gap
    );
}

#[test]
fn test_cert_layout_rows_at_least_one() {
    let layout = cert_layout();
    assert!(layout.rows >= 1, "rows must be >= 1");
}

#[test]
fn test_cert_layout_current_is_horizontal_single_row() {
    let layout = cert_layout();
    assert_eq!(layout.stamp_size, 40);
    assert_eq!(layout.direction, "horizontal");
    assert_eq!(layout.rows, 1);
}

#[test]
fn test_faqs_returns_four() {
    assert_eq!(faqs().len(), 4);
}

#[test]
fn test_inquiry_products_matches_all_products() {
    let product_ids: HashSet<String> = all_products().iter().map(|p| p.id.clone()).collect();
    let inquiry_ids: HashSet<String> = inquiry_products()
        .iter()
        .map(|(id, _)| id.clone())
        .collect();
    assert_eq!(
        product_ids, inquiry_ids,
        "Inquiry product IDs must match catalog product IDs"
    );
}

#[test]
fn test_product_categories_cover_all() {
    let products = all_products();
    let categories: HashSet<String> = products
        .iter()
        .map(|p| p.category.as_str().to_string())
        .collect();
    for expected in &["Shrimp", "Cephalopods", "Fish", "Dried"] {
        assert!(
            categories.contains(*expected),
            "Missing category: {expected}"
        );
    }
}

#[test]
fn test_all_products_have_short_desc() {
    for p in all_products() {
        assert!(
            !p.short_desc.is_empty(),
            "Product '{}' has no short_desc",
            p.name
        );
    }
}

#[test]
fn test_all_products_have_hs_code() {
    for p in all_products() {
        assert!(!p.hs_code.is_empty(), "Product '{}' has no hs_code", p.name);
    }
}
