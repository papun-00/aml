//! TOML frontmatter parser for product markdown files.

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

/// Structured product metadata from markdown frontmatter.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ProductFrontmatter {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
    pub category: String,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub tag: String,
    pub hs_code: String,
    #[serde(default)]
    pub certs: Vec<String>,
    #[serde(default)]
    pub markets: Vec<String>,
    #[serde(default)]
    pub min_order: String,
    #[serde(default)]
    pub short_desc: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub css_class: String,
}

/// Parse a product markdown string into frontmatter + body content.
///
/// Expects `---` delimited TOML frontmatter at the top of the file.
pub fn parse_product(markdown: &str) -> Option<(ProductFrontmatter, String)> {
    let trimmed = markdown.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    // Split on "---" — first part is empty (before first ---), second is frontmatter, rest is body
    let parts: Vec<&str> = trimmed.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }

    let frontmatter_str = parts[1].trim();
    let body = parts[2].trim().to_string();

    let frontmatter: ProductFrontmatter = toml::from_str(frontmatter_str).ok()?;
    Some((frontmatter, body))
}

/// Render markdown content to HTML string using pulldown-cmark.
pub fn render_markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_MD: &str = r#"---
id = "test-product"
name = "Test Product"
scientific_name = "Testus productus"
category = "Fish"
featured = true
tag = "New"
hs_code = "1234.56"
certs = ["HACCP", "EU"]
markets = ["USA", "Japan"]
min_order = "5,000 kg"
short_desc = "A test product."
image_url = "/assets/images/test.svg"
css_class = "test"
---

## Overview

This is a test product.

## Specifications

| Spec | Detail |
|------|--------|
| Size | Large |
"#;

    #[test]
    fn test_parse_product_valid() {
        let result = parse_product(SAMPLE_MD);
        assert!(result.is_some());
        let (fm, body) = result.unwrap();
        assert_eq!(fm.id, "test-product");
        assert_eq!(fm.name, "Test Product");
        assert_eq!(fm.scientific_name, "Testus productus");
        assert_eq!(fm.category, "Fish");
        assert!(fm.featured);
        assert_eq!(fm.tag, "New");
        assert_eq!(fm.hs_code, "1234.56");
        assert_eq!(fm.certs, vec!["HACCP", "EU"]);
        assert_eq!(fm.markets, vec!["USA", "Japan"]);
        assert_eq!(fm.min_order, "5,000 kg");
        assert!(!body.is_empty());
        assert!(body.contains("## Overview"));
    }

    #[test]
    fn test_parse_product_no_frontmatter() {
        let result = parse_product("# Just a heading\nSome content.");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_product_incomplete_frontmatter() {
        let result = parse_product("---\nid = \"incomplete\"\n");
        assert!(result.is_none());
    }

    #[test]
    fn test_render_markdown_to_html() {
        let html = render_markdown_to_html("## Hello\n\nA paragraph with **bold**.");
        assert!(html.contains("<h2>Hello</h2>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_render_markdown_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown_to_html(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<td>1</td>"));
    }
}
