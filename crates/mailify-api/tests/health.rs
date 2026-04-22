//! Smoke test: /health responds 200 OK without any backing services.
//!
//! Queue is not started here — we build the router with a fake queue handle via a narrow helper
//! to avoid pulling Postgres into unit tests. Broader integration tests live under tests/it/ and
//! require docker (enabled by the CI workflow with a `testcontainers` feature).

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    // Minimal router exposing only the public /health route — keeps this test independent
    // of the full app state.
    let app = axum::Router::new().route("/health", axum::routing::get(mailify_api::routes::health::health));

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "mailify");

    // Touch Arc to keep the dependency used and avoid unused-import warnings in future edits.
    let _ = Arc::new(());
}
