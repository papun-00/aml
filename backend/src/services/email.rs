//! Email service — trait-based for testability.
//! Production uses SMTP via `lettre`. Tests use a mock that captures sent emails.

/// Represents an email to be sent.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Trait for sending emails. Implementations: SmtpEmailService (prod), MockEmailService (test).
#[async_trait::async_trait]
pub trait EmailService: Send + Sync {
    async fn send(&self, message: &EmailMessage) -> Result<(), String>;
}

/// Production SMTP email service using lettre.
pub struct SmtpEmailService {
    pub from: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
}

#[async_trait::async_trait]
impl EmailService for SmtpEmailService {
    async fn send(&self, message: &EmailMessage) -> Result<(), String> {
        use lettre::{
            message::header::ContentType, transport::smtp::authentication::Credentials, Message,
            SmtpTransport, Transport,
        };

        let email = Message::builder()
            .from(
                self.from
                    .parse()
                    .map_err(|e| format!("Invalid from: {e}"))?,
            )
            .to(message.to.parse().map_err(|e| format!("Invalid to: {e}"))?)
            .subject(&message.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(message.body.clone())
            .map_err(|e| format!("Failed to build email: {e}"))?;

        let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .map_err(|e| format!("SMTP relay error: {e}"))?
            .credentials(creds)
            .build();

        mailer
            .send(&email)
            .map_err(|e| format!("SMTP send error: {e}"))?;
        Ok(())
    }
}

/// Mock email service for testing — captures all sent emails.
#[derive(Default)]
pub struct MockEmailService {
    pub sent: std::sync::Mutex<Vec<EmailMessage>>,
}

#[async_trait::async_trait]
impl EmailService for MockEmailService {
    async fn send(&self, message: &EmailMessage) -> Result<(), String> {
        self.sent.lock().unwrap().push(message.clone());
        Ok(())
    }
}

/// Build inquiry notification email body.
#[allow(clippy::too_many_arguments)]
pub fn build_inquiry_email(
    notify_to: &str,
    inquiry_id: &str,
    company: &str,
    contact: &str,
    email: &str,
    country: &str,
    products: &[String],
    volume: Option<f32>,
    message: &str,
) -> EmailMessage {
    let product_list = if products.is_empty() {
        "None specified".to_string()
    } else {
        products.join(", ")
    };

    let vol_str = volume
        .map(|v| format!("{v} MT/year"))
        .unwrap_or_else(|| "Not specified".to_string());

    let body = format!(
        "New Inquiry Received\n\
         ════════════════════\n\n\
         Inquiry ID: {inquiry_id}\n\
         Company:    {company}\n\
         Contact:    {contact}\n\
         Email:      {email}\n\
         Country:    {country}\n\
         Products:   {product_list}\n\
         Volume:     {vol_str}\n\n\
         Message:\n{message}\n\n\
         ────────────────────\n\
         Alashore Marine Exports — Automated Notification"
    );

    EmailMessage {
        to: notify_to.to_string(),
        subject: format!("New Inquiry from {company} — {contact}"),
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_inquiry_email_format() {
        let email = build_inquiry_email(
            "export@alashoremarine.com",
            "abc-123",
            "Oceanic Foods",
            "Jane Doe",
            "jane@oceanic.com",
            "USA",
            &["vannamei-shrimp".into(), "squid".into()],
            Some(500.0),
            "Need quarterly supply",
        );

        assert_eq!(email.to, "export@alashoremarine.com");
        assert!(email.subject.contains("Oceanic Foods"));
        assert!(email.subject.contains("Jane Doe"));
        assert!(email.body.contains("abc-123"));
        assert!(email.body.contains("vannamei-shrimp, squid"));
        assert!(email.body.contains("500 MT/year"));
        assert!(email.body.contains("Need quarterly supply"));
    }

    #[test]
    fn test_build_inquiry_email_no_volume() {
        let email = build_inquiry_email(
            "test@test.com",
            "id",
            "Co",
            "Name",
            "e@e.com",
            "UK",
            &[],
            None,
            "",
        );
        assert!(email.body.contains("Not specified"));
        assert!(email.body.contains("None specified"));
    }

    #[tokio::test]
    async fn test_mock_email_service_captures_sent() {
        let mock = MockEmailService::default();
        let msg = EmailMessage {
            to: "test@test.com".into(),
            subject: "Test".into(),
            body: "Body".into(),
        };

        mock.send(&msg).await.unwrap();

        let sent = mock.sent.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].to, "test@test.com");
    }
}
