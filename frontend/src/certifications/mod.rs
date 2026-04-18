//! Certification config module — compile-time embedded TOML config.
//!
//! The single source of truth is `frontend/content/certifications.toml`.

pub mod parser;

pub use parser::{parse_certifications, CertEntry, CertLayout, CertificationsConfig};

/// The TOML source, embedded at compile time.
const CERTS_TOML: &str = include_str!("../../content/certifications.toml");

/// Load and parse the certifications config. Panics if the TOML is malformed
/// (this is intentional — a broken config should fail the build).
pub fn load_config() -> CertificationsConfig {
    parse_certifications(CERTS_TOML)
        .expect("certifications.toml must be valid — check frontend/content/certifications.toml")
}
