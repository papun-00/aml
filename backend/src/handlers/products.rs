use axum::{
    extract::{Path, State},
    Json,
};
use shared::{ApiResponse, Product, ProductCategory, ProductSpec};
use sqlx::SqlitePool;

/// Row from the products table — JSON columns stored as TEXT.
#[derive(sqlx::FromRow)]
struct ProductRow {
    id: String,
    name: String,
    scientific_name: Option<String>,
    category: String,
    tagline: String,
    description: String,
    specifications: String,
    certifications: String,
    markets: String,
    min_order_kg: i32,
    hs_code: String,
    featured: bool,
}

impl ProductRow {
    fn into_product(self) -> Product {
        let category = match self.category.as_str() {
            "shrimp" => ProductCategory::Shrimp,
            "fish" => ProductCategory::Fish,
            "cephalopods" => ProductCategory::Cephalopods,
            "dried" => ProductCategory::Dried,
            _ => ProductCategory::Fish, // safe default
        };

        let specifications: Vec<ProductSpec> =
            serde_json::from_str(&self.specifications).unwrap_or_default();
        let certifications: Vec<String> =
            serde_json::from_str(&self.certifications).unwrap_or_default();
        let markets: Vec<String> = serde_json::from_str(&self.markets).unwrap_or_default();

        Product {
            id: self.id,
            name: self.name,
            scientific_name: self.scientific_name,
            category,
            tagline: self.tagline,
            description: self.description,
            specifications,
            certifications,
            markets,
            min_order_kg: self.min_order_kg as u32,
            hs_code: self.hs_code,
            featured: self.featured,
        }
    }
}

pub async fn list_products(State(pool): State<SqlitePool>) -> Json<ApiResponse<Vec<Product>>> {
    let rows = sqlx::query_as::<_, ProductRow>(
        "SELECT id, name, scientific_name, category, tagline, description,
                specifications, certifications, markets, min_order_kg, hs_code, featured
         FROM products ORDER BY sort_order ASC",
    )
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(rows) => {
            let products: Vec<Product> = rows.into_iter().map(|r| r.into_product()).collect();
            Json(ApiResponse::ok(products))
        }
        Err(e) => {
            tracing::error!("Failed to fetch products: {e}");
            Json(ApiResponse::err("Failed to load products"))
        }
    }
}

pub async fn get_product(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Option<Product>>> {
    let row = sqlx::query_as::<_, ProductRow>(
        "SELECT id, name, scientific_name, category, tagline, description,
                specifications, certifications, markets, min_order_kg, hs_code, featured
         FROM products WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await;

    match row {
        Ok(Some(row)) => Json(ApiResponse::ok(Some(row.into_product()))),
        Ok(None) => Json(ApiResponse::ok(None)),
        Err(e) => {
            tracing::error!("Failed to fetch product {id}: {e}");
            Json(ApiResponse::err("Failed to load product"))
        }
    }
}
