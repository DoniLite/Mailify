//! Integration test — verifies the queue persists jobs in Postgres and that a worker dispatches
//! them all the way through the renderer + SMTP sender (via Mailpit).
//!
//! Skipped unless `MAILIFY_DATABASE__URL` is set and reachable.

use std::sync::Arc;
use std::time::Duration;

use mailify_config::{
    AppConfig, AuthConfig, DatabaseConfig, I18nConfig, LogFormat, ObservabilityConfig, QueueConfig,
    ServerConfig, SmtpConfig, TemplatesConfig, Theme,
};
use mailify_core::{email::EmailAddress, priority::Priority, smtp_override::TlsMode};
use mailify_queue::{
    job::{MailJob, MailJobKind},
    worker::WorkerDeps,
    QueueRuntime,
};
use mailify_smtp::SmtpSender;
use mailify_templates::TemplateRegistry;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

fn database_url() -> Option<String> {
    std::env::var("MAILIFY_DATABASE__URL").ok()
}

fn smtp_host() -> String {
    std::env::var("MAILIFY_SMTP__HOST").unwrap_or_else(|_| "localhost".to_string())
}

fn mailpit_api() -> String {
    std::env::var("MAILPIT_API_URL").unwrap_or_else(|_| "http://localhost:8025".to_string())
}

async fn mailpit_reachable() -> bool {
    let url = format!("{}/api/v1/info", mailpit_api());
    reqwest::get(&url)
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

fn test_cfg(url: String) -> AppConfig {
    AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".into(),
            port: 0,
            request_timeout_secs: 30,
            body_limit_bytes: 1024 * 1024,
        },
        database: DatabaseConfig {
            url,
            max_connections: 4,
            min_connections: 1,
        },
        smtp: SmtpConfig {
            host: smtp_host(),
            port: 1025,
            username: None,
            password: None,
            tls: TlsMode::None,
            default_from_email: "from@mailify.test".into(),
            default_from_name: None,
            timeout_secs: 10,
        },
        auth: AuthConfig {
            jwt_secret: "test".into(),
            jwt_issuer: "mailify".into(),
            jwt_ttl_secs: 60,
            api_keys: Default::default(),
        },
        queue: QueueConfig {
            worker_concurrency: 2,
            max_retries: 0,
            retry_backoff_secs: 1,
        },
        templates: TemplatesConfig {
            path: std::path::PathBuf::from("./out"),
            strict: false,
        },
        theme: Theme::default(),
        i18n: I18nConfig {
            default_locale: "en".into(),
            fallback_chain: vec!["en".into()],
            supported_locales: vec!["en".into()],
        },
        observability: ObservabilityConfig {
            log_level: "warn".into(),
            log_format: LogFormat::Pretty,
        },
    }
}

#[tokio::test]
async fn queue_persists_and_worker_delivers_to_mailpit() {
    let Some(url) = database_url() else {
        eprintln!("SKIP: MAILIFY_DATABASE__URL not set");
        return;
    };
    if !mailpit_reachable().await {
        eprintln!("SKIP: mailpit not reachable");
        return;
    }

    let cfg = test_cfg(url);
    let registry = Arc::new(TemplateRegistry::empty(cfg.i18n.clone()));
    let sender = Arc::new(SmtpSender::from_config(&cfg.smtp).expect("smtp sender"));

    // Purge previous messages so our assertion is unambiguous.
    let _ = reqwest::Client::new()
        .delete(format!("{}/api/v1/messages", mailpit_api()))
        .send()
        .await;

    let (runtime, mut handle) = QueueRuntime::init(
        &cfg,
        WorkerDeps {
            registry: registry.clone(),
            default_sender: sender.clone(),
            theme: cfg.theme.clone(),
        },
    )
    .await
    .expect("queue init");

    let cancel = CancellationToken::new();
    let worker_cancel = cancel.clone();
    let worker = tokio::spawn(async move { runtime.run(worker_cancel).await });

    let marker = format!("mailify-queue-it-{}", Uuid::new_v4());
    let job = MailJob {
        id: Uuid::new_v4(),
        priority: Priority::Critical,
        kind: MailJobKind::Custom {
            html: format!("<p>{marker}</p>"),
            subject: marker.clone(),
            text: None,
        },
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
        attachments: vec![],
        headers: Default::default(),
        locale: "en".into(),
        vars: serde_json::Value::Null,
        smtp_override: None,
        subject_override: None,
    };
    let job_id = handle.push(job).await.expect("push");
    eprintln!("enqueued job {job_id}");

    // Wait for worker to deliver.
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    let search_url = format!(
        "{}/api/v1/search?query={}",
        mailpit_api(),
        urlencoding::encode(&format!("subject:\"{marker}\""))
    );
    let mut delivered = false;
    while std::time::Instant::now() < deadline {
        let resp: serde_json::Value = match reqwest::get(&search_url)
            .await
            .and_then(|r| r.error_for_status())
        {
            Ok(r) => r.json().await.unwrap_or(serde_json::Value::Null),
            Err(_) => {
                tokio::time::sleep(Duration::from_millis(250)).await;
                continue;
            }
        };
        if resp.get("total").and_then(|v| v.as_u64()).unwrap_or(0) > 0 {
            delivered = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    cancel.cancel();
    let _ = tokio::time::timeout(Duration::from_secs(5), worker).await;

    assert!(delivered, "queued job was not delivered to mailpit");
}
