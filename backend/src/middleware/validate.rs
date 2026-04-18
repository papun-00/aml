//! Input validation utilities for OWASP-compliant request handling.

use std::sync::LazyLock;

/// Maximum lengths for input fields to prevent abuse.
pub const MAX_EMAIL_LEN: usize = 254;
pub const MAX_NAME_LEN: usize = 200;
pub const MAX_COMPANY_LEN: usize = 300;
pub const MAX_COUNTRY_LEN: usize = 100;
pub const MAX_PHONE_LEN: usize = 30;
pub const MAX_MESSAGE_LEN: usize = 5000;
pub const MAX_PRODUCT_IDS: usize = 20;

/// Basic email regex — RFC 5322 simplified.
static EMAIL_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*\.[a-zA-Z]{2,}$",
    )
    .unwrap()
});

pub fn is_valid_email(email: &str) -> bool {
    email.len() <= MAX_EMAIL_LEN && EMAIL_RE.is_match(email)
}

/// Strip HTML tags and control chars to prevent stored XSS.
pub fn sanitize(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect::<String>()
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Truncate to max length at a char boundary.
pub fn truncate(input: &str, max: usize) -> String {
    if input.len() <= max {
        input.to_string()
    } else {
        input.chars().take(max).collect()
    }
}

/// Validate and sanitize an inquiry request fields. Returns Err with user-facing message.
pub fn validate_inquiry(
    company: &str,
    name: &str,
    email: &str,
    country: &str,
    product_ids: &[String],
    message: &str,
) -> Result<(), String> {
    if company.trim().is_empty() {
        return Err("Company name is required.".into());
    }
    if company.len() > MAX_COMPANY_LEN {
        return Err("Company name is too long.".into());
    }
    if name.trim().is_empty() {
        return Err("Contact name is required.".into());
    }
    if name.len() > MAX_NAME_LEN {
        return Err("Contact name is too long.".into());
    }
    if !is_valid_email(email) {
        return Err("A valid email address is required.".into());
    }
    if country.trim().is_empty() {
        return Err("Country is required.".into());
    }
    if country.len() > MAX_COUNTRY_LEN {
        return Err("Country name is too long.".into());
    }
    if product_ids.is_empty() {
        return Err("At least one product must be selected.".into());
    }
    if product_ids.len() > MAX_PRODUCT_IDS {
        return Err("Too many products selected.".into());
    }
    if message.len() > MAX_MESSAGE_LEN {
        return Err("Message is too long.".into());
    }
    Ok(())
}

pub fn validate_newsletter(email: &str) -> Result<(), String> {
    if !is_valid_email(email) {
        return Err("A valid email address is required.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(is_valid_email("a@b.cd"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("not-an-email"));
        assert!(!is_valid_email("@domain.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@.com"));
        assert!(!is_valid_email(&"a".repeat(255)));
    }

    #[test]
    fn test_sanitize_strips_html() {
        assert_eq!(
            sanitize("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
        );
        assert_eq!(sanitize("Hello \"World\""), "Hello &quot;World&quot;");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hello");
    }

    #[test]
    fn test_validate_inquiry_empty_company() {
        let result = validate_inquiry("", "John", "j@e.com", "US", &["shrimp".into()], "msg");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_inquiry_invalid_email() {
        let result = validate_inquiry("Co", "John", "bad", "US", &["shrimp".into()], "msg");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_inquiry_valid() {
        let result = validate_inquiry("Co", "John", "j@e.com", "US", &["shrimp".into()], "msg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_newsletter_valid() {
        assert!(validate_newsletter("test@example.com").is_ok());
    }

    #[test]
    fn test_validate_newsletter_invalid() {
        assert!(validate_newsletter("bad").is_err());
    }
}
