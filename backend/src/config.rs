pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub notify_email: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8046),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://data/alashore.db".into()),
            smtp_host: std::env::var("SMTP_HOST").unwrap_or_default(),
            smtp_user: std::env::var("SMTP_USER").unwrap_or_default(),
            smtp_pass: std::env::var("SMTP_PASS").unwrap_or_default(),
            notify_email: std::env::var("NOTIFY_EMAIL")
                .unwrap_or_else(|_| "info@alashoremarine.com".into()),
        }
    }
}
