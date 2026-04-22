use std::sync::Arc;

use axum::{extract::State, Json};
use mailify_core::{
    email::{Attachment, EmailAddress},
    priority::Priority,
    smtp_override::SmtpOverride,
};
use mailify_queue::job::{MailJob, MailJobKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{error::ApiError, state::AppState};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendRegisteredRequest {
    pub template_id: String,
    pub from: Option<EmailAddress>,
    #[validate(length(min = 1, message = "at least one recipient required"))]
    pub to: Vec<EmailAddress>,
    #[serde(default)]
    pub cc: Vec<EmailAddress>,
    #[serde(default)]
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub locale: Option<String>,
    #[serde(default)]
    pub vars: Value,
    #[serde(default)]
    pub priority: Priority,
    pub subject_override: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendCustomRequest {
    pub html: String,
    pub subject: String,
    pub text: Option<String>,
    pub from: Option<EmailAddress>,
    #[validate(length(min = 1, message = "at least one recipient required"))]
    pub to: Vec<EmailAddress>,
    #[serde(default)]
    pub cc: Vec<EmailAddress>,
    #[serde(default)]
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub locale: Option<String>,
    #[serde(default)]
    pub vars: Value,
    #[serde(default)]
    pub priority: Priority,
    /// Caller-supplied SMTP settings. Held in memory only for this job's lifetime; never persisted as plaintext outside the queue store.
    pub smtp_override: Option<SmtpOverride>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnqueuedResponse {
    pub job_id: Uuid,
    pub status: String,
}

/// Queue an email using a built-in template (see `GET /templates` for available ids).
#[utoipa::path(
    post,
    path = "/mail/send",
    tag = "mail",
    request_body = SendRegisteredRequest,
    security(("bearer_jwt" = [])),
    responses(
        (status = 200, description = "Job queued", body = EnqueuedResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Missing or invalid JWT"),
    )
)]
pub async fn send_registered(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendRegisteredRequest>,
) -> Result<Json<EnqueuedResponse>, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let from = default_from(&state, req.from)?;
    let locale = req
        .locale
        .unwrap_or_else(|| state.cfg.i18n.default_locale.clone());

    let job = MailJob {
        id: Uuid::new_v4(),
        priority: req.priority,
        kind: MailJobKind::Registered {
            template_id: req.template_id,
        },
        from,
        to: req.to,
        cc: req.cc,
        bcc: req.bcc,
        reply_to: req.reply_to,
        attachments: req.attachments,
        headers: req.headers,
        locale,
        vars: req.vars,
        smtp_override: None,
        subject_override: req.subject_override,
    };

    let mut queue = state.queue.clone();
    let id = queue
        .push(job)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(EnqueuedResponse {
        job_id: id,
        status: "queued".into(),
    }))
}

/// Queue an email using caller-supplied HTML/subject + optional per-request SMTP override.
/// Override credentials stay in memory only for the job's lifetime — never persisted beyond apalis' job record.
#[utoipa::path(
    post,
    path = "/mail/send-custom",
    tag = "mail",
    request_body = SendCustomRequest,
    security(("bearer_jwt" = [])),
    responses(
        (status = 200, description = "Job queued", body = EnqueuedResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Missing or invalid JWT"),
    )
)]
pub async fn send_custom(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendCustomRequest>,
) -> Result<Json<EnqueuedResponse>, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let from = default_from(&state, req.from)?;
    let locale = req
        .locale
        .unwrap_or_else(|| state.cfg.i18n.default_locale.clone());

    let job = MailJob {
        id: Uuid::new_v4(),
        priority: req.priority,
        kind: MailJobKind::Custom {
            html: req.html,
            subject: req.subject,
            text: req.text,
        },
        from,
        to: req.to,
        cc: req.cc,
        bcc: req.bcc,
        reply_to: req.reply_to,
        attachments: req.attachments,
        headers: req.headers,
        locale,
        vars: req.vars,
        smtp_override: req.smtp_override,
        subject_override: None,
    };

    let mut queue = state.queue.clone();
    let id = queue
        .push(job)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(EnqueuedResponse {
        job_id: id,
        status: "queued".into(),
    }))
}

fn default_from(
    state: &AppState,
    supplied: Option<EmailAddress>,
) -> Result<EmailAddress, ApiError> {
    if let Some(f) = supplied {
        return Ok(f);
    }
    Ok(EmailAddress {
        email: state.cfg.smtp.default_from_email.clone(),
        name: state.cfg.smtp.default_from_name.clone(),
    })
}
