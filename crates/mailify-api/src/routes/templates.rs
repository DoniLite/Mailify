use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use mailify_templates::{RenderContext, TemplateRenderer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::{error::ApiError, state::AppState};

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplateInfo {
    pub id: String,
    pub category: Option<String>,
    pub locales: Vec<String>,
    /// True when a subject sidecar exists for the default locale.
    pub has_subject: bool,
}

/// List all built-in templates with their categories and available locales.
#[utoipa::path(
    get,
    path = "/templates",
    tag = "templates",
    security(("bearer_jwt" = [])),
    responses((status = 200, description = "Template catalog", body = [TemplateInfo]))
)]
pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<TemplateInfo>> {
    let mut grouped: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for key in state.registry.list() {
        grouped.entry(key.id).or_default().push(key.locale);
    }
    let default_locale = state.cfg.i18n.default_locale.clone();
    let out: Vec<TemplateInfo> = grouped
        .into_iter()
        .map(|(id, mut locales)| {
            locales.sort();
            locales.dedup();
            let category = state
                .registry
                .catalog_entry(&id)
                .map(|e| e.category.clone());
            let has_subject = state
                .registry
                .get(&id, &default_locale)
                .map(|a| a.subject.is_some())
                .unwrap_or(false);
            TemplateInfo {
                id,
                category,
                locales,
                has_subject,
            }
        })
        .collect();
    Json(out)
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct PreviewQuery {
    /// BCP-47 locale tag. Falls back via the server's configured fallback chain.
    pub locale: Option<String>,
    /// JSON-encoded vars (GET convenience). Prefer POST for non-trivial payloads.
    pub vars: Option<String>,
    /// When `true`, return JSON `{subject, html, text}`. Default: raw HTML.
    #[serde(default)]
    pub json: bool,
}

#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct PreviewBody {
    pub locale: Option<String>,
    #[serde(default)]
    pub vars: Value,
    #[serde(default)]
    pub json: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PreviewResponse {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

/// Render a registered template for preview. Returns raw HTML by default; set `json=true` for
/// a structured `{subject, html, text}` response.
#[utoipa::path(
    get,
    path = "/templates/{id}/preview",
    tag = "templates",
    security(("bearer_jwt" = [])),
    params(
        ("id" = String, Path, description = "Template id"),
        PreviewQuery,
    ),
    responses(
        (status = 200, description = "Rendered preview", body = PreviewResponse),
        (status = 404, description = "Template not found"),
    )
)]
pub async fn preview_get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(q): Query<PreviewQuery>,
) -> Result<Response, ApiError> {
    let vars: Value = match q.vars.as_deref() {
        Some(s) if !s.is_empty() => serde_json::from_str(s)
            .map_err(|e| ApiError::BadRequest(format!("invalid vars JSON: {e}")))?,
        _ => Value::Null,
    };
    render_preview(&state, &id, q.locale, vars, q.json)
}

/// Render a registered template using a JSON body for locale and vars — richer than the GET form.
#[utoipa::path(
    post,
    path = "/templates/{id}/preview",
    tag = "templates",
    security(("bearer_jwt" = [])),
    params(("id" = String, Path, description = "Template id")),
    request_body = PreviewBody,
    responses(
        (status = 200, description = "Rendered preview", body = PreviewResponse),
        (status = 404, description = "Template not found"),
    )
)]
pub async fn preview_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<PreviewBody>,
) -> Result<Response, ApiError> {
    render_preview(&state, &id, body.locale, body.vars, body.json)
}

fn render_preview(
    state: &AppState,
    id: &str,
    locale: Option<String>,
    vars: Value,
    as_json: bool,
) -> Result<Response, ApiError> {
    let locale = locale.unwrap_or_else(|| state.cfg.i18n.default_locale.clone());
    let ctx = RenderContext {
        theme: state.cfg.theme.clone(),
        locale,
        vars,
    };
    let renderer = TemplateRenderer::new(&state.registry);
    let rendered = renderer
        .render_registered(id, &ctx, None)
        .map_err(|e| match e {
            mailify_templates::renderer::RenderError::Registry(_) => ApiError::NotFound,
            other => ApiError::BadRequest(other.to_string()),
        })?;

    if as_json {
        return Ok((
            StatusCode::OK,
            Json(PreviewResponse {
                subject: rendered.subject,
                html: rendered.html,
                text: rendered.text,
            }),
        )
            .into_response());
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    headers.insert(
        "x-mailify-subject",
        HeaderValue::from_str(&rendered.subject)
            .unwrap_or_else(|_| HeaderValue::from_static("preview")),
    );
    Ok((StatusCode::OK, headers, rendered.html).into_response())
}
