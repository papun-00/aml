//! Single source of truth for all config-driven data in the frontend.

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
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProductConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub scientific_name: &'static str,
    /// Whether the scientific name should be rendered in italics.
    pub sci_italic: bool,
    pub category: ProductCategory,
    pub featured: bool,
    /// Optional marketing tag (e.g. "Popular").
    pub tag: Option<&'static str>,
    pub hs_code: &'static str,
    pub certs: &'static [&'static str],
    pub short_desc: &'static str,
    pub css_class: &'static str,
    pub image_url: &'static str,
}

pub fn all_products() -> Vec<ProductConfig> {
    vec![
        ProductConfig {
            id: "vannamei-shrimp",
            name: "Vannamei Shrimp",
            scientific_name: "Litopenaeus vannamei",
            sci_italic: true,
            category: ProductCategory::Shrimp,
            featured: true,
            tag: None,
            hs_code: "0306.17",
            certs: &["BAP 4\u{2605}", "ASC", "BRC AA"],
            short_desc: "BAP 4-Star certified. Sizes U/10 to 50/60. HOSO, HLSO, PTO, PD, Butterfly, IQF formats.",
            css_class: "vannamei",
            image_url: "/assets/images/vannamei-shrimp.webp",
        },
        ProductConfig {
            id: "black-tiger-shrimp",
            name: "Black Tiger Prawn",
            scientific_name: "Penaeus monodon",
            sci_italic: true,
            category: ProductCategory::Shrimp,
            featured: false,
            tag: None,
            hs_code: "0306.16",
            certs: &["MPEDA", "HACCP", "EU"],
            short_desc: "Wild-caught & farm-raised. Sizes U/8 to 16/20. Sashimi grade available.",
            css_class: "black-tiger",
            image_url: "/assets/images/black-tiger-shrimp.webp",
        },
        ProductConfig {
            id: "squid",
            name: "Indian Squid",
            scientific_name: "Doryteuthis sibogae",
            sci_italic: true,
            category: ProductCategory::Cephalopods,
            featured: true,
            tag: Some("Popular"),
            hs_code: "0307.43",
            certs: &["MPEDA", "HACCP", "EU"],
            short_desc: "Bay of Bengal wild catch. Whole, tubes & tentacles, rings (6mm/10mm), steaks.",
            css_class: "squid",
            image_url: "/assets/images/squid.webp",
        },
        ProductConfig {
            id: "cuttlefish",
            name: "Cuttlefish",
            scientific_name: "Sepia pharaonis",
            sci_italic: true,
            category: ProductCategory::Cephalopods,
            featured: false,
            tag: None,
            hs_code: "0307.99",
            certs: &["MPEDA", "HACCP", "EU"],
            short_desc: "Pharaoh cuttlefish from Bay of Bengal. Whole cleaned or tubes & tentacles.",
            css_class: "cuttlefish",
            image_url: "/assets/images/cuttlefish.webp",
        },
        ProductConfig {
            id: "pink-perch",
            name: "Pink Perch",
            scientific_name: "Nemipterus japonicus",
            sci_italic: true,
            category: ProductCategory::Fish,
            featured: false,
            tag: None,
            hs_code: "0302.89",
            certs: &["MPEDA", "HACCP", "EU"],
            short_desc: "Threadfin bream. Whole round, HG, fillet skin-on or skinless.",
            css_class: "pink-perch",
            image_url: "/assets/images/pink-perch.webp",
        },
        ProductConfig {
            id: "dried-shrimp",
            name: "Dried Shrimp",
            scientific_name: "Sun-Dried",
            sci_italic: false,
            category: ProductCategory::Dried,
            featured: false,
            tag: None,
            hs_code: "0306.99",
            certs: &["FSSAI", "MPEDA"],
            short_desc: "No preservatives. Whole, crushed or powder. 12-month shelf life at +4\u{00b0}C.",
            css_class: "dried-shrimp",
            image_url: "/assets/images/dried-shrimp.webp",
        },
    ]
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
        StatItem { label: "Peak Revenue",       value: "₹436 Cr",  desc: "FY2020 turnover" },
        StatItem { label: "Credit Rating",      value: "A3+",       desc: "CRISIL short-term" },
        StatItem { label: "LC-Backed Revenue",  value: "95%+",      desc: "Letter of Credit secured" },
    ]
}

// ---------------------------------------------------------------------------
// Certifications
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub struct CertConfig {
    pub name: &'static str,
    pub icon: &'static str,
}

pub fn certifications() -> Vec<CertConfig> {
    vec![
        CertConfig { name: "BAP 4-Star", icon: "\u{2605}\u{2605}\u{2605}\u{2605}" },
        CertConfig { name: "BRC AA",     icon: "\u{2713}" },
        CertConfig { name: "ASC",        icon: "\u{2713}" },
        CertConfig { name: "HACCP",      icon: "\u{2713}" },
        CertConfig { name: "EU Approved", icon: "\u{2713}" },
    ]
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
            answer: "Yes. Alashore Marine Exports holds a CRISIL A3+ short-term credit rating, backed by a ₹100 Crore credit facility from The Federal Bank. The company's current ratio stands at 2.16x with peak revenue of ₹436 Crore (FY2020).",
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

pub fn inquiry_products() -> Vec<(&'static str, &'static str)> {
    vec![
        ("vannamei-shrimp", "Vannamei Whiteleg Shrimp"),
        ("black-tiger-shrimp", "Black Tiger Prawn"),
        ("squid", "Indian Squid"),
        ("cuttlefish", "Cuttlefish"),
        ("pink-perch", "Pink Perch (Threadfin Bream)"),
        ("dried-shrimp", "Sun-Dried Shrimp"),
    ]
}
