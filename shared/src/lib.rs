use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProductCategory {
    Shrimp,
    Fish,
    Cephalopods,
    Dried,
}

impl ProductCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Shrimp => "Shrimp & Prawns",
            Self::Fish => "Fish",
            Self::Cephalopods => "Cephalopods",
            Self::Dried => "Dried Seafood",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub scientific_name: Option<String>,
    pub category: ProductCategory,
    pub tagline: String,
    pub description: String,
    pub specifications: Vec<ProductSpec>,
    pub certifications: Vec<String>,
    pub markets: Vec<String>,
    pub min_order_kg: u32,
    pub hs_code: String,
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSpec {
    pub size_grade: String,
    pub forms: Vec<String>,
    pub glaze_percent: Option<u8>,
    pub shelf_life_months: u8,
    pub temp_celsius: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InquiryRequest {
    pub company_name: String,
    pub contact_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub country: String,
    pub product_ids: Vec<String>,
    pub volume_mt_per_year: Option<f32>,
    pub message: String,
    pub preferred_contact: ContactMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactMethod {
    Email,
    Phone,
    WhatsApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self { success: false, data: None, error: Some(msg.into()) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterRequest {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoMeta {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub og_image: Option<String>,
    pub canonical: String,
    pub schema_type: SchemaType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaType {
    Organization,
    Product,
    WebPage,
    BreadcrumbList,
}
