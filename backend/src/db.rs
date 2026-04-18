use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // Ensure data dir exists for sqlite file path
    let db_path = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"));
    if let Some(path) = db_path {
        let path = path.split('?').next().unwrap_or(path);
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                tokio::fs::create_dir_all(parent).await.ok();
            }
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    // Run schema inline (avoids sqlx offline mode requirement)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS inquiries (
            id           TEXT PRIMARY KEY,
            company_name TEXT NOT NULL,
            contact_name TEXT NOT NULL,
            email        TEXT NOT NULL,
            phone        TEXT NOT NULL DEFAULT '',
            country      TEXT NOT NULL,
            product_ids  TEXT NOT NULL DEFAULT '[]',
            volume_mt    REAL,
            message      TEXT NOT NULL,
            status       TEXT NOT NULL DEFAULT 'new',
            created_at   TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS newsletter_subscribers (
            email         TEXT PRIMARY KEY,
            name          TEXT NOT NULL DEFAULT '',
            subscribed_at TEXT NOT NULL,
            active        INTEGER NOT NULL DEFAULT 1
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id              TEXT PRIMARY KEY,
            name            TEXT NOT NULL,
            scientific_name TEXT,
            category        TEXT NOT NULL,
            tagline         TEXT NOT NULL,
            description     TEXT NOT NULL,
            specifications  TEXT NOT NULL DEFAULT '[]',
            certifications  TEXT NOT NULL DEFAULT '[]',
            markets         TEXT NOT NULL DEFAULT '[]',
            min_order_kg    INTEGER NOT NULL DEFAULT 0,
            hs_code         TEXT NOT NULL,
            featured        INTEGER NOT NULL DEFAULT 0,
            sort_order      INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS audit_log (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp   TEXT NOT NULL,
            method      TEXT NOT NULL,
            endpoint    TEXT NOT NULL,
            status_code INTEGER NOT NULL,
            ip          TEXT NOT NULL DEFAULT '',
            user_agent  TEXT NOT NULL DEFAULT ''
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_inquiries_created ON inquiries(created_at DESC)")
        .execute(&pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_log_ts ON audit_log(timestamp DESC)")
        .execute(&pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_sort ON products(sort_order ASC)")
        .execute(&pool)
        .await
        .ok();

    // Seed products if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await?;

    if count.0 == 0 {
        seed_products(&pool).await?;
    }

    tracing::info!("✅ Database ready");
    Ok(pool)
}

async fn seed_products(pool: &SqlitePool) -> anyhow::Result<()> {
    let products = product_seed_data();
    for (i, p) in products.iter().enumerate() {
        sqlx::query(
            "INSERT INTO products (id, name, scientific_name, category, tagline, description,
             specifications, certifications, markets, min_order_kg, hs_code, featured, sort_order)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(p.id)
        .bind(p.name)
        .bind(p.scientific_name)
        .bind(p.category)
        .bind(p.tagline)
        .bind(p.description)
        .bind(&p.specifications)
        .bind(&p.certifications)
        .bind(&p.markets)
        .bind(p.min_order_kg)
        .bind(p.hs_code)
        .bind(p.featured)
        .bind(i as i32)
        .execute(pool)
        .await?;
    }
    tracing::info!("🌱 Seeded {} products", products.len());
    Ok(())
}

struct ProductSeed {
    id: &'static str,
    name: &'static str,
    scientific_name: Option<&'static str>,
    category: &'static str,
    tagline: &'static str,
    description: &'static str,
    specifications: String,
    certifications: String,
    markets: String,
    min_order_kg: i32,
    hs_code: &'static str,
    featured: bool,
}

fn product_seed_data() -> Vec<ProductSeed> {
    vec![
        ProductSeed {
            id: "vannamei-shrimp",
            name: "Whiteleg Shrimp",
            scientific_name: Some("Litopenaeus vannamei"),
            category: "shrimp",
            tagline: "The World's Most Traded Shrimp",
            description: "Premium aquaculture-raised Vannamei shrimp from BAP-certified farms in Odisha. Available in HOSO, HLSO, PTO, PD, and IQF butterfly formats. Fully traceable from pond to port.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "U/10, 10/20, 20/30", "forms": ["HOSO","HLSO","PTO","PD","Butterfly"], "glaze_percent": 0, "shelf_life_months": 24, "temp_celsius": -18}),
                serde_json::json!({"size_grade": "30/40, 40/50, 50/60", "forms": ["HLSO","PTO","EZ-Peel","Skewered"], "glaze_percent": 10, "shelf_life_months": 24, "temp_celsius": -18}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["BAP 4-Star","ASC","BRC AA","HACCP","EU Approved"]).unwrap(),
            markets: serde_json::to_string(&["USA","EU","Japan","South Korea","Australia"]).unwrap(),
            min_order_kg: 20_000,
            hs_code: "0306.17",
            featured: true,
        },
        ProductSeed {
            id: "black-tiger-shrimp",
            name: "Black Tiger Shrimp",
            scientific_name: Some("Penaeus monodon"),
            category: "shrimp",
            tagline: "Wild-Caught & Farm-Raised Premium Grade",
            description: "Giant tiger prawns sourced from Bay of Bengal waters and contracted farms. Prized for their bold flavour, firm texture and impressive size. Export-grade processing under EU HACCP standards.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "U/8, U/10, 13/15, 16/20", "forms": ["HOSO","HLSO","PTO","PD","Sashimi Grade"], "glaze_percent": 0, "shelf_life_months": 24, "temp_celsius": -18}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["MPEDA","HACCP","EU Approved","FSSAI"]).unwrap(),
            markets: serde_json::to_string(&["Japan","USA","UAE","EU"]).unwrap(),
            min_order_kg: 5_000,
            hs_code: "0306.16",
            featured: true,
        },
        ProductSeed {
            id: "pink-perch",
            name: "Pink Perch (Threadfin Bream)",
            scientific_name: Some("Nemipterus japonicus"),
            category: "fish",
            tagline: "Mild, White-Fleshed Bay of Bengal Catch",
            description: "Fresh-caught Pink Perch processed within 4 hours of landing. Whole round, gutted, HG or filleted. Primary surimi-grade and retail-grade available. Cold-chain guaranteed from Balasore.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "100-200g, 200-300g, 300-500g, 500g+", "forms": ["Whole Round","Gutted & Gilled","HG","Fillet Skin-On","Fillet Skinless"], "glaze_percent": null, "shelf_life_months": 18, "temp_celsius": -18}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["MPEDA","HACCP","FSSAI","EU Approved"]).unwrap(),
            markets: serde_json::to_string(&["China","Southeast Asia","Middle East","EU"]).unwrap(),
            min_order_kg: 10_000,
            hs_code: "0302.89",
            featured: false,
        },
        ProductSeed {
            id: "squid",
            name: "Indian Squid",
            scientific_name: Some("Doryteuthis sibogae"),
            category: "cephalopods",
            tagline: "Bay of Bengal Wild Catch – Pristine & Fast-Frozen",
            description: "Wild-caught Indian squid, frozen within 2 hours of catch aboard vessels or at shore plants. Available as whole cleaned, tubes & tentacles, rings, or steaks. High yield, consistent sizing.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "100-200g, 200-300g, 300-500g", "forms": ["Whole Uncleaned","Whole Cleaned","Tubes & Tentacles","Rings (6mm / 10mm)","Steaks"], "glaze_percent": 5, "shelf_life_months": 24, "temp_celsius": -18}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["MPEDA","HACCP","EU Approved"]).unwrap(),
            markets: serde_json::to_string(&["Spain","Italy","South Korea","Japan","UAE"]).unwrap(),
            min_order_kg: 10_000,
            hs_code: "0307.43",
            featured: true,
        },
        ProductSeed {
            id: "cuttlefish",
            name: "Cuttlefish",
            scientific_name: Some("Sepia pharaonis"),
            category: "cephalopods",
            tagline: "Pharaoh Cuttlefish – Premium Wild Catch",
            description: "Pharaoh cuttlefish from the Bay of Bengal. Whole or cleaned, with ink sac intact on request. Highly prized in Mediterranean and East Asian markets. Processed under strict HACCP controls.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "100-200g, 200-400g, 400g+", "forms": ["Whole Uncleaned","Whole Cleaned","Tubes & Tentacles"], "glaze_percent": 5, "shelf_life_months": 24, "temp_celsius": -18}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["MPEDA","HACCP","EU Approved"]).unwrap(),
            markets: serde_json::to_string(&["Spain","Portugal","Japan","South Korea"]).unwrap(),
            min_order_kg: 5_000,
            hs_code: "0307.99",
            featured: false,
        },
        ProductSeed {
            id: "dried-shrimp",
            name: "Sun-Dried Shrimp",
            scientific_name: None,
            category: "dried",
            tagline: "Traditional Sun-Dried, Low Moisture, Long Shelf Life",
            description: "Premium sun-dried shrimp from contracted Bay of Bengal catches. Low-moisture processing, no preservatives. Packed in food-grade HDPE bags or custom retail packaging. Popular in Southeast Asian, African and Middle Eastern markets.",
            specifications: serde_json::to_string(&vec![
                serde_json::json!({"size_grade": "Whole Small / Whole Medium / Crushed", "forms": ["Whole Dried","Crushed","Powder"], "glaze_percent": null, "shelf_life_months": 12, "temp_celsius": 4}),
            ]).unwrap(),
            certifications: serde_json::to_string(&["FSSAI","MPEDA","HACCP"]).unwrap(),
            markets: serde_json::to_string(&["Southeast Asia","Middle East","Africa","USA"]).unwrap(),
            min_order_kg: 2_000,
            hs_code: "0306.99",
            featured: false,
        },
    ]
}
