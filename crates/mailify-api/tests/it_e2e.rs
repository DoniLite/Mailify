//! End-to-end integration test — spins up the full axum app (router, queue, SMTP), issues a JWT,
//! POSTs `/mail/send-custom`, and verifies the message hits Mailpit.
//!
//! Skipped unless `MAILIFY_DATABASE__URL` is set *and* Mailpit is reachable.

use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use mailify_auth::JwtIssuer;
use mailify_config::{
    AppConfig, AuthConfig, DatabaseConfig, I18nConfig, LogFormat, ObservabilityConfig, QueueConfig,
    ServerConfig, SmtpConfig, TemplatesConfig, Theme,
};
use mailify_core::smtp_override::TlsMode;
use mailify_queue::{worker::WorkerDeps, QueueRuntime};
use mailify_smtp::SmtpSender;
use mailify_templates::TemplateRegistry;
use tokio_util::sync::CancellationToken;
use tower::ServiceExt;

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
    reqwest::get(format!("{}/api/v1/info", mailpit_api()))
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

fn build_test_cfg(url: String) -> AppConfig {
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
            default_from_name: Some("Mailify E2E".into()),
            timeout_secs: 10,
        },
        auth: AuthConfig {
            jwt_secret: "e2e-secret".into(),
            jwt_issuer: "mailify-e2e".into(),
            jwt_ttl_secs: 300,
            api_keys: Default::default(),
            bootstrap: false,
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
async fn send_custom_via_http_delivers_to_mailpit() {
    let Some(url) = database_url() else {
        eprintln!("SKIP: MAILIFY_DATABASE__URL not set");
        return;
    };
    if !mailpit_reachable().await {
        eprintln!("SKIP: mailpit not reachable");
        return;
    }

    // Purge previous messages.
    let _ = reqwest::Client::new()
        .delete(format!("{}/api/v1/messages", mailpit_api()))
        .send()
        .await;

    let cfg = build_test_cfg(url);
    let registry = Arc::new(TemplateRegistry::empty(cfg.i18n.clone()));
    let sender = Arc::new(SmtpSender::from_config(&cfg.smtp).expect("smtp"));

    let (runtime, queue_handle) = QueueRuntime::init(
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

    let jwt_issuer = Arc::new(JwtIssuer::new(
        cfg.auth.jwt_secret.clone(),
        cfg.auth.jwt_issuer.clone(),
        cfg.auth.jwt_ttl_secs,
    ));
    let token = jwt_issuer.issue("e2e-test", vec![]).expect("issue");

    let state = mailify_api::AppState {
        cfg: Arc::new(cfg.clone()),
        registry: registry.clone(),
        queue: queue_handle,
        jwt: jwt_issuer,
    };
    let app = mailify_api::build_router(state);

    let marker = format!("mailify-e2e-{}", uuid::Uuid::new_v4());
    let body = serde_json::json!({
        "html": format!("<p>{marker}</p>"),
        "subject": marker,
        "to": [{"email": "recipient@mailify.test"}],
        "priority": "critical"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mail/send-custom")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .expect("http");

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 8 * 1024)
        .await
        .unwrap();
    let resp: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(resp["status"], "pending");

    // Wait for mailpit delivery.
    let search_url = format!(
        "{}/api/v1/search?query={}",
        mailpit_api(),
        urlencoding::encode(&format!("subject:\"{marker}\""))
    );
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
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

    assert!(delivered, "e2e: email not delivered to mailpit");
}

#[tokio::test]
async fn protected_routes_reject_missing_token() {
    let app = mailify_api::build_router(mailify_api::AppState {
        cfg: Arc::new(build_test_cfg("postgres://x/x".into())),
        registry: Arc::new(TemplateRegistry::empty(I18nConfig {
            default_locale: "en".into(),
            fallback_chain: vec!["en".into()],
            supported_locales: vec!["en".into()],
        })),
        queue: {
            // Can't create a real QueueHandle without postgres, so skip if no DB.
            let Some(url) = database_url() else {
                eprintln!("SKIP: MAILIFY_DATABASE__URL not set");
                return;
            };
            let cfg = build_test_cfg(url);
            let (_rt, handle) = QueueRuntime::init(
                &cfg,
                WorkerDeps {
                    registry: Arc::new(TemplateRegistry::empty(cfg.i18n.clone())),
                    default_sender: Arc::new(SmtpSender::from_config(&cfg.smtp).expect("smtp")),
                    theme: cfg.theme.clone(),
                },
            )
            .await
            .expect("queue init");
            handle
        },
        jwt: Arc::new(JwtIssuer::new("s", "mailify", 60)),
    });

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/config")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let resp = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/templates")
                .header("authorization", "Bearer bogus.jwt.value")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
