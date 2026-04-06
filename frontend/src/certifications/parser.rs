//! TOML parser for the certifications config file.

use serde::Deserialize;

/// Top-level certifications config from TOML.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CertificationsConfig {
    pub layout: CertLayout,
    pub certs: Vec<CertEntry>,
}

/// Layout settings — controls how badges are rendered in the strip.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CertLayout {
    /// "horizontal" or "vertical"
    pub direction: String,
    /// Number of rows (1 = single line)
    pub rows: u8,
    /// Uniform badge size in px
    pub stamp_size: u16,
    /// Gap between badges in px
    pub gap: u16,
}

/// A single certification entry.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CertEntry {
    pub id: String,
    pub name: String,
    /// "inline" (SVG rendered in code) or "image" (external file path)
    pub badge_type: String,
    /// Hex brand color for the badge background
    pub brand_color: String,
    /// Short text rendered on the badge (top line)
    pub label_short: String,
    /// Optional second line text
    #[serde(default)]
    pub label_line2: String,
    /// Fallback icon text (emoji/unicode)
    #[serde(default)]
    pub icon_text: String,
    /// Image path (only used when badge_type = "image")
    #[serde(default)]
    pub image_path: String,
}

/// Parse the certifications TOML string into structured config.
pub fn parse_certifications(toml_str: &str) -> Result<CertificationsConfig, String> {
    toml::from_str(toml_str).map_err(|e| format!("Failed to parse certifications TOML: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_TOML: &str = r##"
[layout]
direction = "horizontal"
rows = 1
stamp_size = 32
gap = 12

[[certs]]
id = "test"
name = "Test Cert"
badge_type = "inline"
brand_color = "#FF0000"
label_short = "TEST"
icon_text = "✓"
"##;

    #[test]
    fn test_parse_minimal_config() {
        let config = parse_certifications(MINIMAL_TOML).unwrap();
        assert_eq!(config.layout.direction, "horizontal");
        assert_eq!(config.layout.rows, 1);
        assert_eq!(config.layout.stamp_size, 32);
        assert_eq!(config.layout.gap, 12);
        assert_eq!(config.certs.len(), 1);
        assert_eq!(config.certs[0].id, "test");
    }

    #[test]
    fn test_parse_layout_fields() {
        let config = parse_certifications(MINIMAL_TOML).unwrap();
        assert_eq!(config.layout.direction, "horizontal");
        assert_eq!(config.layout.stamp_size, 32);
    }

    #[test]
    fn test_parse_cert_entry_fields() {
        let config = parse_certifications(MINIMAL_TOML).unwrap();
        let cert = &config.certs[0];
        assert_eq!(cert.name, "Test Cert");
        assert_eq!(cert.badge_type, "inline");
        assert_eq!(cert.brand_color, "#FF0000");
        assert_eq!(cert.label_short, "TEST");
        assert_eq!(cert.icon_text, "✓");
        assert!(cert.label_line2.is_empty());
        assert!(cert.image_path.is_empty());
    }

    #[test]
    fn test_parse_multiple_certs() {
        let toml = r##"
[layout]
direction = "vertical"
rows = 2
stamp_size = 48
gap = 16

[[certs]]
id = "a"
name = "Cert A"
badge_type = "inline"
brand_color = "#111111"
label_short = "A"

[[certs]]
id = "b"
name = "Cert B"
badge_type = "image"
brand_color = "#222222"
label_short = "B"
image_path = "/assets/images/cert-b.png"
"##;
        let config = parse_certifications(toml).unwrap();
        assert_eq!(config.certs.len(), 2);
        assert_eq!(config.layout.direction, "vertical");
        assert_eq!(config.layout.rows, 2);
        assert_eq!(config.certs[1].badge_type, "image");
        assert_eq!(config.certs[1].image_path, "/assets/images/cert-b.png");
    }

    #[test]
    fn test_parse_invalid_toml_returns_error() {
        let result = parse_certifications("not valid toml {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_field_returns_error() {
        let toml = r##"
[layout]
direction = "horizontal"
rows = 1
stamp_size = 32
gap = 12

[[certs]]
id = "missing-name"
badge_type = "inline"
brand_color = "#000"
label_short = "X"
"##;
        let result = parse_certifications(toml);
        assert!(result.is_err(), "Should fail when 'name' is missing");
    }

    #[test]
    fn test_label_line2_defaults_empty() {
        let config = parse_certifications(MINIMAL_TOML).unwrap();
        assert_eq!(config.certs[0].label_line2, "");
    }
}
