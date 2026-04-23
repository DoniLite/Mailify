//! Integration test — sends a real email through Mailpit and verifies it arrived via Mailpit's API.
//!
//! Skipped automatically unless both are reachable:
//!   MAILIFY_SMTP__HOST  (defaults to localhost)
//!   MAILPIT_API_URL     (defaults to http://localhost:8025)
//!
//! CI sets these via docker-compose services. Locally, run `make up-deps`.

use mailify_config::SmtpConfig;
use mailify_core::{
    email::{EmailAddress, RenderedEmail},
    smtp_override::TlsMode,
};
use mailify_smtp::{Envelope, SmtpSender};

fn smtp_host() -> String {
    std::env::var("MAILIFY_SMTP__HOST").unwrap_or_else(|_| "localhost".to_string())
}

fn mailpit_api() -> String {
    std::env::var("MAILPIT_API_URL").unwrap_or_else(|_| "http://localhost:8025".to_string())
}

async fn mailpit_reachable() -> bool {
    let url = format!("{}/api/v1/info", mailpit_api());
    match reqwest::get(&url).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

async fn purge_mailpit() {
    let _ = reqwest::Client::new()
        .delete(format!("{}/api/v1/messages", mailpit_api()))
        .send()
        .await;
}

#[tokio::test]
async fn sends_email_through_mailpit_end_to_end() {
    if !mailpit_reachable().await {
        eprintln!("SKIP: mailpit not reachable at {}", mailpit_api());
        return;
    }

    purge_mailpit().await;

    let cfg = SmtpConfig {
        host: smtp_host(),
        port: 1025,
        username: None,
        password: None,
        tls: TlsMode::None,
        default_from_email: "from@mailify.test".into(),
        default_from_name: Some("Mailify CI".into()),
        timeout_secs: 10,
    };
    let sender = SmtpSender::from_config(&cfg).expect("build sender");

    let marker = format!("mailify-it-{}", uuid::Uuid::new_v4());
    let envelope = Envelope {
        from: EmailAddress {
            email: "from@mailify.test".into(),
            name: None,
        },
        to: vec![EmailAddress {
            email: "to@mailify.test".into(),
            name: None,
        }],
        cc: vec![],
        bcc: vec![],
        reply_to: None,
        headers: Default::default(),
        attachments: vec![],
    };
    let rendered = RenderedEmail {
        subject: marker.clone(),
        html: format!("<p>{marker}</p>"),
        text: Some(marker.clone()),
    };

    sender.send(&envelope, &rendered).await.expect("send");

    // Poll mailpit API for the message. Mailpit ingests within milliseconds but give some slack.
    let url = format!(
        "{}/api/v1/search?query={}",
        mailpit_api(),
        urlencoding::encode(&format!("subject:\"{marker}\""))
    );

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let mut found = false;
    while std::time::Instant::now() < deadline {
        let resp: serde_json::Value = reqwest::get(&url)
            .await
            .expect("query mailpit")
            .json()
            .await
            .expect("parse json");
        if resp
            .get("total")
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            > 0
        {
            found = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    assert!(found, "email with subject {marker} not delivered to mailpit");
}
