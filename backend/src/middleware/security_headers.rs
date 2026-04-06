//! OWASP-compliant security headers middleware.

use axum::{
    middleware::Next,
    http::{Request, HeaderValue},
    response::Response,
    body::Body,
};

/// Injects security headers on every response per OWASP recommendations.
pub async fn security_headers(
    request: Request<Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Prevent MIME-sniffing
    headers.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    // Clickjacking protection
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    // XSS filter (legacy browsers)
    headers.insert("x-xss-protection", HeaderValue::from_static("1; mode=block"));
    // Referrer policy
    headers.insert("referrer-policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    // Permissions policy — restrict sensitive APIs
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
    );
    // HSTS (effective behind TLS terminator)
    headers.insert(
        "strict-transport-security",
        HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
    );
    // Content Security Policy
    headers.insert(
        "content-security-policy",
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' https://images.unsplash.com data: https:; connect-src 'self' ws: wss:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'"
        ),
    );

    response
}
