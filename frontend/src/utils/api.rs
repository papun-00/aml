//! API client for communicating with the Alashore backend.
//! Config-driven: API_BASE_URL defaults to "/api/v1" (same-origin).

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use shared::ApiResponse;

const API_BASE: &str = "/api/v1";

/// POST JSON to a backend endpoint and deserialize the ApiResponse.
pub async fn post_json<Req: Serialize, Res: DeserializeOwned>(
    path: &str,
    body: &Req,
) -> Result<ApiResponse<Res>, String> {
    let url = format!("{API_BASE}{path}");
    let resp = Request::post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(body).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if resp.status() == 429 {
        return Err("Too many requests. Please wait and try again.".into());
    }

    resp.json::<ApiResponse<Res>>()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

/// Submit an inquiry to the backend.
#[allow(clippy::too_many_arguments)]
pub async fn submit_inquiry(
    company: &str,
    name: &str,
    email: &str,
    phone: &str,
    country: &str,
    products: &[String],
    volume: &str,
    incoterms: &str,
    message: &str,
) -> Result<String, String> {
    let volume_mt: Option<f32> = volume.parse().ok();

    let body = serde_json::json!({
        "company_name": company,
        "contact_name": name,
        "email": email,
        "phone": if phone.is_empty() { None } else { Some(phone) },
        "country": country,
        "product_ids": products,
        "volume_mt_per_year": volume_mt,
        "message": format!("{}{}", message, if incoterms.is_empty() { String::new() } else { format!("\n\nPreferred Incoterms: {incoterms}") }),
        "preferred_contact": "email"
    });

    let resp: ApiResponse<String> = post_json("/inquiry", &body).await?;

    if resp.success {
        Ok(resp.data.unwrap_or_default())
    } else {
        Err(resp.error.unwrap_or_else(|| "Unknown error".into()))
    }
}

/// Subscribe to the newsletter.
#[allow(dead_code)]
pub async fn subscribe_newsletter(email: &str, name: &str) -> Result<String, String> {
    let body = serde_json::json!({
        "email": email,
        "name": if name.is_empty() { None } else { Some(name) }
    });

    let resp: ApiResponse<String> = post_json("/newsletter", &body).await?;

    if resp.success {
        Ok(resp.data.unwrap_or_default())
    } else {
        Err(resp.error.unwrap_or_else(|| "Unknown error".into()))
    }
}
