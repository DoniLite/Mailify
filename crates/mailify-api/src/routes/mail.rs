use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use mailify_core::{
    email::{Attachment, EmailAddress},
    priority::Priority,
    smtp_override::SmtpOverride,
};
use mailify_queue::{
    job::{MailJob, MailJobKind},
    JobSnapshot,
};
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
    /// Queue task id (ULID). Use it with `GET /mail/jobs/{id}` to poll state.
    pub job_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobStateResponse {
    pub task_id: String,
    pub mail_id: Uuid,
    /// One of: `pending`, `scheduled`, `running`, `done`, `failed`, `killed`.
    pub status: String,
    pub attempts: usize,
    pub max_attempts: i32,
    pub last_error: Option<String>,
    pub run_at: chrono::DateTime<chrono::Utc>,
    pub lock_at: Option<i64>,
    pub done_at: Option<i64>,
}

impl From<JobSnapshot> for JobStateResponse {
    fn from(s: JobSnapshot) -> Self {
        Self {
            task_id: s.task_id,
            mail_id: s.mail_id,
            status: s.status,
            attempts: s.attempts,
            max_attempts: s.max_attempts,
            last_error: s.last_error,
            run_at: s.run_at,
            lock_at: s.lock_at,
            done_at: s.done_at,
        }
    }
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
        status: "pending".into(),
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
        status: "pending".into(),
    }))
}

/// Fetch the current state of a previously-enqueued job.
#[utoipa::path(
    get,
    path = "/mail/jobs/{id}",
    tag = "mail",
    params(("id" = String, Path, description = "Task id returned by /mail/send or /mail/send-custom")),
    security(("bearer_jwt" = [])),
    responses(
        (status = 200, description = "Job state", body = JobStateResponse),
        (status = 400, description = "Malformed id"),
        (status = 401, description = "Missing or invalid JWT"),
        (status = 404, description = "Unknown job id (or vacuumed)"),
    )
)]
pub async fn get_job_state(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JobStateResponse>, ApiError> {
    let mut queue = state.queue.clone();
    match queue.fetch(&id).await {
        Ok(Some(snap)) => Ok(Json(snap.into())),
        Ok(None) => Err(ApiError::NotFound),
        Err(mailify_queue::worker::QueueError::InvalidId(m)) => Err(ApiError::BadRequest(m)),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
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
