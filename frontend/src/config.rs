//! Single source of truth for all config-driven data in the frontend.
//!
//! Product data is derived from markdown files in `frontend/content/products/`.
//! All other data (stats, certs, nav, FAQs, company info) remains here.

use crate::products;

// ---------------------------------------------------------------------------
// Product Catalog
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub enum ProductCategory {
    Shrimp,
    Cephalopods,
    Fish,
    Dried,
}

impl ProductCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductCategory::Shrimp => "Shrimp",
            ProductCategory::Cephalopods => "Cephalopods",
            ProductCategory::Fish => "Fish",
            ProductCategory::Dried => "Dried",
        }
    }

    pub fn from_label(s: &str) -> Self {
        match s {
            "Shrimp" => ProductCategory::Shrimp,
            "Cephalopods" => ProductCategory::Cephalopods,
            "Fish" => ProductCategory::Fish,
            "Dried" => ProductCategory::Dried,
            _ => ProductCategory::Fish,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProductConfig {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
    pub category: ProductCategory,
    pub featured: bool,
    pub tag: Option<String>,
    pub hs_code: String,
    pub certs: Vec<String>,
    pub markets: Vec<String>,
    pub min_order: String,
    pub short_desc: String,
    pub css_class: String,
    pub image_url: String,
}

/// Derive all products from embedded markdown frontmatter.
pub fn all_products() -> Vec<ProductConfig> {
    products::all_parsed_products()
        .into_iter()
        .map(|(fm, _body)| ProductConfig {
            id: fm.id,
            name: fm.name,
            scientific_name: fm.scientific_name,
            category: ProductCategory::from_label(&fm.category),
            featured: fm.featured,
            tag: if fm.tag.is_empty() { None } else { Some(fm.tag) },
            hs_code: fm.hs_code,
            certs: fm.certs,
            markets: fm.markets,
            min_order: fm.min_order,
            short_desc: fm.short_desc,
            css_class: fm.css_class,
            image_url: fm.image_url,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct StatItem {
    pub label: &'static str,
    pub value: &'static str,
    pub desc: &'static str,
}

pub fn company_stats() -> Vec<StatItem> {
    vec![
        StatItem { label: "Founded",           value: "2012",      desc: "December 2012, Balasore" },
        StatItem { label: "Export Markets",     value: "30+",       desc: "Countries worldwide" },
        StatItem { label: "Workforce",          value: "800+",      desc: "Direct employees" },
        StatItem { label: "Peak Revenue",       value: "\u{20b9}436 Cr",  desc: "FY2020 turnover" },
        StatItem { label: "Credit Rating",      value: "A3+",       desc: "CRISIL short-term" },
        StatItem { label: "LC-Backed Revenue",  value: "95%+",      desc: "Letter of Credit secured" },
    ]
}

// ---------------------------------------------------------------------------
// Certifications — derived from frontend/content/certifications.toml
// ---------------------------------------------------------------------------

pub use crate::certifications::{CertificationsConfig, CertLayout, CertEntry, load_config as load_cert_config};

/// Load the full certifications config (layout + entries) from TOML.
pub fn cert_config() -> CertificationsConfig {
    load_cert_config()
}

/// Convenience: returns just the cert entries list.
pub fn certifications() -> Vec<CertEntry> {
    load_cert_config().certs
}

/// Convenience: returns just the layout settings.
pub fn cert_layout() -> CertLayout {
    load_cert_config().layout
}

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct NavItem {
    pub label: &'static str,
    pub route_name: &'static str,
}

pub fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem { label: "Home",            route_name: "HomePage" },
        NavItem { label: "About",           route_name: "AboutPage" },
        NavItem { label: "Products",        route_name: "ProductsPage" },
        NavItem { label: "Certifications",  route_name: "CertificationsPage" },
        NavItem { label: "Sustainability",  route_name: "SustainabilityPage" },
        NavItem { label: "Contact",         route_name: "ContactPage" },
        NavItem { label: "Inquiry",         route_name: "InquiryPage" },
    ]
}

// ---------------------------------------------------------------------------
// FAQs
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct FaqItem {
    pub question: &'static str,
    pub answer: &'static str,
}

pub fn faqs() -> Vec<FaqItem> {
    vec![
        FaqItem {
            question: "What seafood products does Alashore Marine export?",
            answer: "Alashore Marine Exports exports Vannamei (Whiteleg) shrimp in sizes U/10 to 50/60, Black Tiger prawns (U/8 to 16/20), Indian squid (whole, tubes & tentacles, rings, steaks), Pharaoh cuttlefish, Pink Perch (Threadfin Bream), and sun-dried shrimp. All products carry HACCP and EU Approval certifications.",
        },
        FaqItem {
            question: "Is Alashore Marine Exports financially rated?",
            answer: "Yes. Alashore Marine Exports holds a CRISIL A3+ short-term credit rating, backed by a \u{20b9}100 Crore credit facility from The Federal Bank. The company's current ratio stands at 2.16x with peak revenue of \u{20b9}436 Crore (FY2020).",
        },
        FaqItem {
            question: "What certifications does Alashore Marine hold?",
            answer: "BAP 4-Star (covering farm, hatchery, feed mill, and processing plant), BRC Grade AA, ASC (Aquaculture Stewardship Council), HACCP, EU Establishment Approval, MPEDA registration, FSSAI, and CRISIL A3+ credit rating.",
        },
        FaqItem {
            question: "Which countries does Alashore Marine supply to?",
            answer: "Alashore Marine exports to 30+ countries including the USA, all 27 EU member states, Japan, South Korea, Australia, UAE, Saudi Arabia, and Southeast Asia. Over 95% of export revenue is secured by Letters of Credit.",
        },
    ]
}

// ---------------------------------------------------------------------------
// Growth Milestones (Journey Timeline)
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct GrowthMilestone {
    pub year: u16,
    pub title: &'static str,
    pub description: &'static str,
    pub metric_label: Option<&'static str>,
    pub metric_value: Option<&'static str>,
    pub icon: &'static str,
    pub highlight: bool,
}

pub fn growth_milestones() -> Vec<GrowthMilestone> {
    vec![
        GrowthMilestone {
            year: 2012,
            title: "Founded in Balasore",
            description: "Incorporated as Accenture Marine Exports. First processing unit at Somnathpur Industrial Estate, Odisha.",
            metric_label: Some("Capacity"),
            metric_value: Some("20 MT/day"),
            icon: "anchor",
            highlight: false,
        },
        GrowthMilestone {
            year: 2014,
            title: "EU Market Entry",
            description: "Secured MPEDA registration and EU Establishment Approval. First shipments to Spain and Italy.",
            metric_label: Some("Markets"),
            metric_value: Some("5+"),
            icon: "globe",
            highlight: false,
        },
        GrowthMilestone {
            year: 2016,
            title: "BAP 4-Star & USA",
            description: "Achieved BAP 4-Star certification. Commenced exports to the United States. Capacity scaled to 80 MT/day.",
            metric_label: Some("Capacity"),
            metric_value: Some("80 MT/day"),
            icon: "star",
            highlight: false,
        },
        GrowthMilestone {
            year: 2018,
            title: "BRC AA & ASC Certified",
            description: "BRC Grade AA for food safety. ASC certification for sustainability. In-house EIA-approved lab opened.",
            metric_label: Some("Certifications"),
            metric_value: Some("6+"),
            icon: "shield",
            highlight: false,
        },
        GrowthMilestone {
            year: 2019,
            title: "Rebranded to Alashore",
            description: "Rebranded to Alashore Marine Exports Pvt. Ltd. Secured \u{20b9}100 Crore Federal Bank credit facility.",
            metric_label: Some("Credit"),
            metric_value: Some("\u{20b9}100 Cr"),
            icon: "refresh",
            highlight: false,
        },
        GrowthMilestone {
            year: 2020,
            title: "Peak Revenue",
            description: "Record turnover of \u{20b9}436 Crore. 800+ workforce. 150 MT/day capacity. CRISIL A3+ rating.",
            metric_label: Some("Revenue"),
            metric_value: Some("\u{20b9}436 Cr"),
            icon: "trending-up",
            highlight: true,
        },
        GrowthMilestone {
            year: 2024,
            title: "Global Expansion",
            description: "Exporting to 30+ countries. Ongoing vertical integration investments. Digital infrastructure launched.",
            metric_label: Some("Markets"),
            metric_value: Some("30+"),
            icon: "globe",
            highlight: false,
        },
    ]
}

// ---------------------------------------------------------------------------
// Company Info
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct CompanyInfo {
    pub name: &'static str,
    pub tagline: &'static str,
    pub founded: u16,
    pub founder: &'static str,
    pub address: &'static str,
    pub email_info: &'static str,
    pub email_export: &'static str,
    pub email_quality: &'static str,
    pub latitude: f64,
    pub longitude: f64,
    pub iec_code: &'static str,
    pub gstin: &'static str,
    pub fda_fceid: &'static str,
}

pub const COMPANY: CompanyInfo = CompanyInfo {
    name: "Alashore Marine Exports Pvt. Ltd.",
    tagline: "India's Premier Frozen Seafood Exporter",
    founded: 2012,
    founder: "Gyan Ranjan Dash",
    address: "Plot No. 123, Seafood Park, Balasore, Odisha 756001, India",
    email_info: "info@alashoremarine.com",
    email_export: "export@alashoremarine.com",
    email_quality: "quality@alashoremarine.com",
    latitude: 21.4942,
    longitude: 86.9337,
    iec_code: "",
    gstin: "",
    fda_fceid: "",
};

// ---------------------------------------------------------------------------
// Product Inquiry List
// ---------------------------------------------------------------------------

pub fn inquiry_products() -> Vec<(String, String)> {
    all_products()
        .into_iter()
        .map(|p| (p.id, p.name))
        .collect()
}
