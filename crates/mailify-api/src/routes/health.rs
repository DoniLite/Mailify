use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

/// Liveness probe. Always returns `{"status":"ok","service":"mailify"}` when the HTTP server is up.
#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses((status = 200, description = "Server alive", body = HealthResponse))
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        service: "mailify".into(),
    })
}
