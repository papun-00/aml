#![allow(non_snake_case)]
//! SEO module — zero-markup + GenAI (AEO) optimised metadata injection.
//!
//! Dual-strategy:
//!   1. Traditional: title, description, canonical, OG, Twitter, keywords
//!   2. GenAI / AEO: entity meta, speakable, citation hints, JSON-LD schemas
//!      — ensures ChatGPT, Perplexity, Gemini, Claude, Bing Copilot cite us correctly

use dioxus::prelude::*;

/// Core SEO props shared by all pages
#[derive(Props, Clone, PartialEq)]
pub struct SeoProps {
    pub title: String,
    pub description: String,
    #[props(default)]
    pub keywords: Option<String>,
    #[props(default)]
    pub canonical: Option<String>,
    #[props(default)]
    pub og_image: Option<String>,
    /// Raw JSON-LD string — injected per page
    #[props(default)]
    pub schema_json: Option<String>,
    /// Speakable plain-text summary — used by voice search + AI extraction
    #[props(default)]
    pub speakable: Option<String>,
    /// Page type for breadcrumb schema
    #[props(default)]
    pub breadcrumbs: Vec<BreadcrumbItem>,
}

#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub name: String,
    pub url: String,
}

impl BreadcrumbItem {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self { name: name.into(), url: url.into() }
    }
}

const BASE_URL: &str = "https://alashoremarine.com";
const DEFAULT_OG: &str = "https://alashoremarine.com/assets/og-card.jpg";
const ORG_NAME: &str  = "Alashore Marine Exports";

/// Full SEO head injection — call at top of every page component
#[component]
pub fn PageSeo(props: SeoProps) -> Element {
    let full_title = format!("{} | {} – Premium Seafood Exporter India", props.title, ORG_NAME);
    let canonical = props.canonical
        .clone()
        .unwrap_or_else(|| BASE_URL.to_string());
    let og_image = props.og_image
        .clone()
        .unwrap_or_else(|| DEFAULT_OG.to_string());

    // Build breadcrumb JSON-LD
    let breadcrumb_json = if !props.breadcrumbs.is_empty() {
        let items = props.breadcrumbs
            .iter()
            .enumerate()
            .map(|(i, b)| {
                format!(
                    r#"{{"@type":"ListItem","position":{},"name":"{}","item":"{}"}}"#,
                    i + 1, b.name, b.url
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Some(format!(
            r#"{{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{}]}}"#,
            items
        ))
    } else {
        None
    };

    rsx! {
        // ── Primary SEO ──────────────────────────────────────────────
        document::Title { "{full_title}" }
        document::Meta { name: "description",      content: "{props.description}" }
        if let Some(kw) = &props.keywords {
            document::Meta { name: "keywords", content: "{kw}" }
        }
        document::Meta { name: "robots",
            content: "index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1" }
        document::Link { rel: "canonical", href: "{canonical}" }

        // ── Open Graph ───────────────────────────────────────────────
        document::Meta { property: "og:type",        content: "website" }
        document::Meta { property: "og:url",         content: "{canonical}" }
        document::Meta { property: "og:site_name",   content: "{ORG_NAME}" }
        document::Meta { property: "og:title",       content: "{full_title}" }
        document::Meta { property: "og:description", content: "{props.description}" }
        document::Meta { property: "og:image",       content: "{og_image}" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height",content: "630" }
        document::Meta { property: "og:locale",      content: "en_US" }

        // ── Twitter/X ────────────────────────────────────────────────
        document::Meta { name: "twitter:card",        content: "summary_large_image" }
        document::Meta { name: "twitter:title",       content: "{full_title}" }
        document::Meta { name: "twitter:description", content: "{props.description}" }
        document::Meta { name: "twitter:image",       content: "{og_image}" }

        // ── GenAI / AEO signals ──────────────────────────────────────
        // Citation hints for LLM citation systems (Perplexity, Bing AI, ChatGPT)
        document::Meta { name: "citation_title",      content: "{full_title}" }
        document::Meta { name: "citation_author",     content: "{ORG_NAME}" }
        document::Meta { name: "citation_publisher",  content: "alashoremarine.com" }
        document::Meta { name: "citation_public_url", content: "{canonical}" }

        // Speakable content — voice search + AI answer extraction
        if let Some(spk) = &props.speakable {
            document::Meta { name: "speakable", content: "{spk}" }
        }

        // Entity signals for AI knowledge graph disambiguation
        document::Meta { name: "entity:name",     content: "Alashore Marine Exports Pvt. Ltd." }
        document::Meta { name: "entity:type",     content: "Organization" }
        document::Meta { name: "entity:location", content: "Balasore, Odisha, India" }
        document::Meta { name: "entity:industry", content: "Seafood Export, Aquaculture" }

        // ── JSON-LD schema (page-specific) ───────────────────────────
        if let Some(schema) = &props.schema_json {
            document::Script {
                r#type: "application/ld+json",
                dangerous_inner_html: "{schema}"
            }
        }

        // ── Breadcrumb JSON-LD ───────────────────────────────────────
        if let Some(bc) = breadcrumb_json {
            document::Script {
                r#type: "application/ld+json",
                dangerous_inner_html: "{bc}"
            }
        }
    }
}

// ── Pre-built per-page SEO configs ────────────────────────────────────────

pub fn home_seo() -> SeoProps {
    SeoProps {
        title: "India's Premier Frozen Seafood Exporter".into(),
        description: "Alashore Marine Exports — BAP 4-Star, BRC AA & CRISIL A3+ rated. Exporting premium frozen Vannamei shrimp, Black Tiger prawns, squid and fish from Balasore, Odisha to 30+ countries since 2012.".into(),
        keywords: Some("seafood exporter India, shrimp exporter Odisha, frozen shrimp supplier India, Vannamei shrimp export India, marine exports Balasore, BAP certified seafood India".into()),
        canonical: Some("https://alashoremarine.com/".into()),
        og_image: None,
        schema_json: Some(home_schema()),
        speakable: Some("Alashore Marine Exports is a CRISIL A3+ rated frozen seafood exporter from Balasore, Odisha, India. Founded in 2012, the company exports BAP 4-Star certified Vannamei shrimp, Black Tiger prawns, squid, and fish to over 30 countries. Key certifications include BRC AA, ASC, HACCP, and EU establishment approval.".into()),
        breadcrumbs: vec![BreadcrumbItem::new("Home", "https://alashoremarine.com/")],
    }
}

pub fn about_seo() -> SeoProps {
    SeoProps {
        title: "About Us – Our Story Since 2012".into(),
        description: "Learn about Alashore Marine Exports: founded December 2012 in Balasore, Odisha by Mr. Gyan Ranjan Dash. 800+ employees, CRISIL A3+ rated, ₹436 Cr peak revenue. EU-approved processing with in-house EIA lab.".into(),
        keywords: Some("Alashore Marine Exports history, Gyan Ranjan Dash, seafood exporter Odisha India, CRISIL rated seafood company, marine export company India".into()),
        canonical: Some("https://alashoremarine.com/about".into()),
        og_image: None,
        schema_json: Some(about_schema()),
        speakable: Some("Alashore Marine Exports was founded in December 2012 by Mr. Gyan Ranjan Dash. The company is based in Balasore, Odisha and employs over 800 people. It holds a CRISIL A3+ credit rating and reached peak revenue of ₹436 Crore in FY2020.".into()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("About", "https://alashoremarine.com/about"),
        ],
    }
}

pub fn products_seo() -> SeoProps {
    SeoProps {
        title: "Products – Frozen Shrimp, Squid, Fish & More".into(),
        description: "Browse Alashore Marine's full product range: Vannamei shrimp (U/10–50/60), Black Tiger prawns, Indian squid, Pharaoh cuttlefish, Pink Perch, dried shrimp. All EU-approved, HACCP & BAP certified.".into(),
        keywords: Some("buy frozen shrimp India, Vannamei shrimp wholesale, Black Tiger prawn supplier, squid exporter India, frozen cephalopods supplier, HACCP certified seafood wholesale".into()),
        canonical: Some("https://alashoremarine.com/products".into()),
        og_image: None,
        schema_json: None,
        speakable: Some("Alashore Marine Exports offers six main product categories: Vannamei whiteleg shrimp in sizes U/10 to 50/60, Black Tiger prawns, Indian squid, Pharaoh cuttlefish, Pink Perch threadfin bream, and sun-dried shrimp. All products are HACCP certified and EU approved.".into()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Products", "https://alashoremarine.com/products"),
        ],
    }
}

pub fn certifications_seo() -> SeoProps {
    SeoProps {
        title: "Certifications – BAP, BRC AA, ASC, HACCP, EU Approved".into(),
        description: "Alashore Marine Exports holds BAP 4-Star, BRC Grade AA, ASC, HACCP, EU Establishment Approval, MPEDA, FSSAI and CRISIL A3+ certifications. Full details and audit validity dates.".into(),
        keywords: Some("BAP 4 star certified seafood India, BRC AA seafood exporter, ASC certified shrimp, HACCP seafood India, EU approved seafood establishment India, MPEDA registered exporter".into()),
        canonical: Some("https://alashoremarine.com/certifications".into()),
        og_image: None,
        schema_json: None,
        speakable: Some("Alashore Marine Exports holds the following certifications: BAP 4-Star covering farm, hatchery, feed mill and processing plant; BRC Grade AA for food safety; ASC for aquaculture sustainability; HACCP; EU Establishment Approval for the European market; MPEDA registration; FSSAI; and CRISIL A3+ credit rating.".into()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Certifications", "https://alashoremarine.com/certifications"),
        ],
    }
}

pub fn sustainability_seo() -> SeoProps {
    SeoProps {
        title: "Sustainability – ASC, BAP & Responsible Aquaculture".into(),
        description: "Alashore Marine's sustainability programme: ASC and BAP certified, zero-mangrove-clearance, recirculating aquaculture systems, vertical integration from hatchery to processing, EIA-compliant wastewater treatment.".into(),
        keywords: Some("sustainable seafood India, ASC certified shrimp, BAP certified aquaculture, responsible seafood exporter India, sustainable aquaculture Odisha".into()),
        canonical: Some("https://alashoremarine.com/sustainability".into()),
        og_image: None,
        schema_json: None,
        speakable: Some("Alashore Marine Exports operates a sustainability programme certified under ASC and BAP standards, with a zero-mangrove-clearance policy, recirculating aquaculture systems, and vertical integration from hatchery to processing plant.".into()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Sustainability", "https://alashoremarine.com/sustainability"),
        ],
    }
}

pub fn contact_seo() -> SeoProps {
    SeoProps {
        title: "Contact Us – Get in Touch".into(),
        description: "Contact Alashore Marine Exports at Somnathpur Industrial Estate, Balasore, Odisha 756019, India. Email: info@alashoremarine.com. For export inquiries: export@alashoremarine.com.".into(),
        keywords: Some("contact Alashore Marine Exports, seafood exporter contact India, Balasore seafood company contact".into()),
        canonical: Some("https://alashoremarine.com/contact".into()),
        og_image: None,
        schema_json: Some(contact_schema()),
        speakable: Some("Contact Alashore Marine Exports at Plot D1/18-D1/39, Somnathpur Industrial Estate, Balasore, Odisha 756019, India. General enquiries: info@alashoremarine.com. Export sales: export@alashoremarine.com.".into()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Contact", "https://alashoremarine.com/contact"),
        ],
    }
}

pub fn inquiry_seo() -> SeoProps {
    SeoProps {
        title: "Request a Quote – B2B Seafood Inquiry".into(),
        description: "Submit a B2B seafood inquiry to Alashore Marine Exports. Specify product, volume, target market and incoterms. Receive a competitive quote within 24 hours.".into(),
        keywords: Some("seafood price quote India, buy shrimp wholesale India, frozen seafood bulk supplier quote, B2B seafood inquiry India".into()),
        canonical: Some("https://alashoremarine.com/inquiry".into()),
        og_image: None,
        schema_json: None,
        speakable: None,
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Request a Quote", "https://alashoremarine.com/inquiry"),
        ],
    }
}

/// Generate product-specific SEO props
pub fn product_seo(id: &str, name: &str, description: &str, scientific: Option<&str>) -> SeoProps {
    let sci_str = scientific.map(|s| format!(" ({s})")).unwrap_or_default();
    let full_canonical = format!("https://alashoremarine.com/products/{id}");
    let schema = product_schema(id, name, description);

    SeoProps {
        title: format!("{name}{sci_str} – India Export Supplier"),
        description: description.to_string(),
        keywords: Some(format!(
            "{name} exporter India, buy {name} wholesale, frozen {name} supplier India, {name} India export price"
        )),
        canonical: Some(full_canonical.clone()),
        og_image: None,
        schema_json: Some(schema),
        speakable: Some(description.to_string()),
        breadcrumbs: vec![
            BreadcrumbItem::new("Home", "https://alashoremarine.com/"),
            BreadcrumbItem::new("Products", "https://alashoremarine.com/products"),
            BreadcrumbItem::new(name, full_canonical),
        ],
    }
}

// ── Schema builders ────────────────────────────────────────────────────────

fn home_schema() -> String {
    r#"{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "Organization",
      "@id": "https://alashoremarine.com/#organization",
      "name": "Alashore Marine Exports Pvt. Ltd.",
      "url": "https://alashoremarine.com",
      "foundingDate": "2012-12-01",
      "founders": [{"@type":"Person","name":"Gyan Ranjan Dash"}],
      "numberOfEmployees": {"@type":"QuantitativeValue","value":800},
      "address": {
        "@type": "PostalAddress",
        "streetAddress": "Plot D1/18-D1/39, Somnathpur Industrial Estate",
        "addressLocality": "Balasore", "addressRegion": "Odisha",
        "postalCode": "756019", "addressCountry": "IN"
      },
      "knowsAbout": ["Seafood Export","Frozen Shrimp","Aquaculture","Vannamei Shrimp","Black Tiger Prawn","Marine Products"]
    },
    {
      "@type": "WebPage",
      "@id": "https://alashoremarine.com/#webpage",
      "url": "https://alashoremarine.com/",
      "name": "Alashore Marine Exports | India's Premier Seafood Exporter",
      "isPartOf": {"@id": "https://alashoremarine.com/#website"},
      "about": {"@id": "https://alashoremarine.com/#organization"},
      "description": "BAP 4-Star, BRC AA and CRISIL A3+ rated frozen seafood exporter from Balasore, Odisha, India."
    }
  ]
}"#.to_string()
}

fn about_schema() -> String {
    r#"{
  "@context": "https://schema.org",
  "@type": "AboutPage",
  "url": "https://alashoremarine.com/about",
  "name": "About Alashore Marine Exports",
  "mainEntity": {
    "@type": "Organization",
    "@id": "https://alashoremarine.com/#organization",
    "name": "Alashore Marine Exports Pvt. Ltd.",
    "foundingDate": "2012-12-01",
    "founders": [{"@type":"Person","name":"Gyan Ranjan Dash","jobTitle":"Managing Director"}]
  }
}"#.to_string()
}

fn contact_schema() -> String {
    r#"{
  "@context": "https://schema.org",
  "@type": "ContactPage",
  "url": "https://alashoremarine.com/contact",
  "name": "Contact Alashore Marine Exports",
  "mainEntity": {
    "@type": "Organization",
    "@id": "https://alashoremarine.com/#organization",
    "contactPoint": [
      {
        "@type": "ContactPoint",
        "contactType": "sales",
        "email": "export@alashoremarine.com",
        "availableLanguage": "English",
        "areaServed": "Worldwide"
      },
      {
        "@type": "ContactPoint",
        "contactType": "customer service",
        "email": "info@alashoremarine.com",
        "availableLanguage": ["English","Hindi","Odia"]
      }
    ]
  }
}"#.to_string()
}

fn product_schema(id: &str, name: &str, description: &str) -> String {
    format!(r#"{{
  "@context": "https://schema.org",
  "@type": "Product",
  "@id": "https://alashoremarine.com/products/{id}",
  "name": "{name}",
  "description": "{description}",
  "brand": {{"@type": "Brand", "name": "Alashore Marine"}},
  "manufacturer": {{"@id": "https://alashoremarine.com/#organization"}},
  "countryOfOrigin": "India",
  "category": "Frozen Seafood",
  "offers": {{
    "@type": "Offer",
    "availability": "https://schema.org/InStock",
    "priceCurrency": "USD",
    "seller": {{"@id": "https://alashoremarine.com/#organization"}}
  }}
}}"#)
}
