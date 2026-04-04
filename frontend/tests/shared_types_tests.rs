use shared::*;

#[test]
fn test_product_category_label() {
    assert_eq!(ProductCategory::Shrimp.label(), "Shrimp & Prawns");
    assert_eq!(ProductCategory::Fish.label(), "Fish");
    assert_eq!(ProductCategory::Cephalopods.label(), "Cephalopods");
    assert_eq!(ProductCategory::Dried.label(), "Dried Seafood");
}

#[test]
fn test_api_response_ok() {
    let resp = ApiResponse::ok(42u32);
    assert!(resp.success);
    assert_eq!(resp.data, Some(42));
    assert!(resp.error.is_none());
}

#[test]
fn test_api_response_err() {
    let resp = ApiResponse::<()>::err("something failed");
    assert!(!resp.success);
    assert!(resp.data.is_none());
    assert_eq!(resp.error.as_deref(), Some("something failed"));
}

#[test]
fn test_inquiry_request_serialization() {
    let req = InquiryRequest {
        company_name: "Acme Corp".into(),
        contact_name: "John Doe".into(),
        email: "john@acme.com".into(),
        phone: Some("+1234567890".into()),
        country: "USA".into(),
        product_ids: vec!["vannamei-shrimp".into()],
        volume_mt_per_year: Some(100.0),
        message: "Need quote".into(),
        preferred_contact: ContactMethod::Email,
    };
    let json = serde_json::to_string(&req).unwrap();
    let deserialized: InquiryRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.company_name, "Acme Corp");
    assert_eq!(deserialized.email, "john@acme.com");
    assert_eq!(deserialized.product_ids, vec!["vannamei-shrimp"]);
}

#[test]
fn test_newsletter_request_serialization() {
    let req = NewsletterRequest {
        email: "reader@example.com".into(),
        name: Some("Jane".into()),
    };
    let json = serde_json::to_string(&req).unwrap();
    let deserialized: NewsletterRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.email, "reader@example.com");
    assert_eq!(deserialized.name.as_deref(), Some("Jane"));
}
