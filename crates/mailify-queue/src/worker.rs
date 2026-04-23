use std::sync::Arc;
use std::time::Duration;

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_sql::context::SqlContext;
use apalis_sql::postgres::{PgListen, PostgresStorage};
use chrono::{DateTime, Utc};
use mailify_config::{AppConfig, QueueConfig};
use mailify_core::smtp_override::SmtpOverride;
use mailify_smtp::{Envelope, SmtpSender};
use mailify_templates::{RenderContext, TemplateRegistry, TemplateRenderer};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info};

use crate::job::{MailJob, MailJobKind};

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("apalis migration error: {0}")]
    Migrate(String),
    #[error("apalis push error: {0}")]
    Push(String),
    #[error("invalid task id: {0}")]
    InvalidId(String),
    #[error("apalis fetch error: {0}")]
    Fetch(String),
    #[error("worker error: {0}")]
    Worker(String),
}

/// Point-in-time view of a queued job's state, as exposed to HTTP clients.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JobSnapshot {
    /// apalis task id (ULID) — the externally-visible job identifier.
    pub task_id: String,
    /// Our domain-level MailJob id (useful to correlate logs).
    pub mail_id: uuid::Uuid,
    /// One of: pending, scheduled, running, done, failed, killed.
    pub status: String,
    pub attempts: usize,
    pub max_attempts: i32,
    pub last_error: Option<String>,
    pub run_at: DateTime<Utc>,
    /// Unix seconds when the job was locked by a worker.
    pub lock_at: Option<i64>,
    /// Unix seconds when the job finished (either Done or Failed).
    pub done_at: Option<i64>,
}

/// Handle used by HTTP routes to enqueue and inspect jobs.
#[derive(Clone)]
pub struct QueueHandle {
    storage: PostgresStorage<MailJob>,
}

impl QueueHandle {
    /// Enqueue a job and return its apalis task id (ULID string).
    pub async fn push(&mut self, job: MailJob) -> Result<String, QueueError> {
        let parts = self
            .storage
            .push(job)
            .await
            .map_err(|e| QueueError::Push(e.to_string()))?;
        Ok(parts.task_id.to_string())
    }

    /// Fetch a snapshot of a job's current state. Returns `None` if the id is unknown
    /// or the job has been vacuumed out of storage.
    pub async fn fetch(&mut self, task_id: &str) -> Result<Option<JobSnapshot>, QueueError> {
        let parsed: TaskId = task_id
            .parse()
            .map_err(|e| QueueError::InvalidId(format!("{e}")))?;
        let res = self
            .storage
            .fetch_by_id(&parsed)
            .await
            .map_err(|e| QueueError::Fetch(e.to_string()))?;
        Ok(res.map(|req| snapshot_from_request(task_id, &req)))
    }
}

fn snapshot_from_request(
    task_id: &str,
    req: &apalis::prelude::Request<MailJob, SqlContext>,
) -> JobSnapshot {
    let ctx = &req.parts.context;
    JobSnapshot {
        task_id: task_id.to_string(),
        mail_id: req.args.id,
        status: state_label(ctx.status()).to_string(),
        attempts: req.parts.attempt.current(),
        max_attempts: ctx.max_attempts(),
        last_error: ctx.last_error().clone(),
        run_at: *ctx.run_at(),
        lock_at: *ctx.lock_at(),
        done_at: *ctx.done_at(),
    }
}

fn state_label(s: &apalis::prelude::State) -> &'static str {
    use apalis::prelude::State;
    match s {
        State::Pending => "pending",
        State::Scheduled => "scheduled",
        State::Running => "running",
        State::Done => "done",
        State::Failed => "failed",
        State::Killed => "killed",
    }
}

/// Owns the apalis runtime. `run()` spawns workers and blocks until shutdown.
pub struct QueueRuntime {
    cfg: QueueConfig,
    storage: PostgresStorage<MailJob>,
    pool: sqlx::PgPool,
    deps: Arc<WorkerDeps>,
}

pub struct WorkerDeps {
    pub registry: Arc<TemplateRegistry>,
    pub default_sender: Arc<SmtpSender>,
    pub theme: mailify_config::Theme,
}

impl QueueRuntime {
    pub async fn init(
        app_cfg: &AppConfig,
        deps: WorkerDeps,
    ) -> Result<(Self, QueueHandle), QueueError> {
        let sanitized = redact_db_url(&app_cfg.database.url);
        info!(
            database.url = %sanitized,
            database.max_connections = app_cfg.database.max_connections,
            database.min_connections = app_cfg.database.min_connections,
            "connecting to postgres",
        );

        let pool = PgPoolOptions::new()
            .max_connections(app_cfg.database.max_connections)
            .min_connections(app_cfg.database.min_connections)
            .acquire_timeout(Duration::from_secs(10))
            .connect(&app_cfg.database.url)
            .await
            .map_err(|e| {
                error!(error = %e, database.url = %sanitized, "postgres connection failed");
                QueueError::Sqlx(e)
            })?;

        sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
            error!(error = %e, "postgres ping (SELECT 1) failed");
            QueueError::Sqlx(e)
        })?;
        info!("postgres reachable");

        PostgresStorage::setup(&pool).await.map_err(|e| {
            error!(error = %e, "apalis migrations failed");
            QueueError::Migrate(e.to_string())
        })?;
        info!("apalis migrations applied");

        let storage = PostgresStorage::new(pool.clone());
        let handle = QueueHandle {
            storage: storage.clone(),
        };

        Ok((
            Self {
                cfg: app_cfg.queue.clone(),
                storage,
                pool,
                deps: Arc::new(deps),
            },
            handle,
        ))
    }

    /// Spawn workers. Returns when shutdown signal received.
    pub async fn run(
        self,
        shutdown: tokio_util::sync::CancellationToken,
    ) -> Result<(), QueueError> {
        let deps = self.deps.clone();
        let concurrency = self.cfg.worker_concurrency;
        let retries = self.cfg.max_retries;

        info!(concurrency, retries, "starting mail queue workers");

        // Postgres LISTEN/NOTIFY for near-instant job pickup.
        let mut listener = PgListen::new(self.pool.clone())
            .await
            .map_err(|e| QueueError::Worker(e.to_string()))?;

        let storage = self.storage.clone();
        listener.subscribe_with(&mut storage.clone());

        tokio::spawn(async move {
            if let Err(e) = listener.listen().await {
                error!(error = %e, "pg listener terminated");
            }
        });

        let monitor = Monitor::new().register({
            WorkerBuilder::new("mailify-worker")
                .concurrency(concurrency)
                .retry(RetryPolicy::retries(retries))
                .data(deps)
                .backend(storage)
                .build_fn(handle_job)
        });

        tokio::select! {
            res = monitor.run() => res.map_err(|e| QueueError::Worker(e.to_string()))?,
            _ = shutdown.cancelled() => {
                info!("queue shutdown requested");
            }
        }
        Ok(())
    }
}

async fn handle_job(job: MailJob, deps: Data<Arc<WorkerDeps>>) -> Result<(), Error> {
    let deps: Arc<WorkerDeps> = (*deps).clone();
    process(job, deps)
        .await
        .map_err(|e| Error::Failed(Arc::new(Box::new(std::io::Error::other(e)))))
}

async fn process(job: MailJob, deps: Arc<WorkerDeps>) -> Result<(), String> {
    let renderer = TemplateRenderer::new(&deps.registry);
    let ctx = RenderContext {
        theme: deps.theme.clone(),
        locale: job.locale.clone(),
        vars: job.vars.clone(),
    };

    let rendered = match &job.kind {
        MailJobKind::Registered { template_id } => renderer
            .render_registered(template_id, &ctx, job.subject_override.as_deref())
            .map_err(|e| e.to_string())?,
        MailJobKind::Custom {
            html,
            subject,
            text,
        } => renderer
            .render_raw(
                html,
                job.subject_override.as_deref().unwrap_or(subject),
                text.as_deref(),
                &ctx,
            )
            .map_err(|e| e.to_string())?,
    };

    let envelope = Envelope {
        from: job.from.clone(),
        to: job.to.clone(),
        cc: job.cc.clone(),
        bcc: job.bcc.clone(),
        reply_to: job.reply_to.clone(),
        headers: job.headers.clone(),
        attachments: job.attachments.clone(),
    };

    let sender = match &job.smtp_override {
        Some(ov) => Arc::new(sender_from_override(ov).map_err(|e| e.to_string())?),
        None => deps.default_sender.clone(),
    };

    sender
        .send(&envelope, &rendered)
        .await
        .map_err(|e| e.to_string())?;
    info!(job_id = %job.id, "mail sent");
    Ok(())
}

fn sender_from_override(ov: &SmtpOverride) -> Result<SmtpSender, mailify_smtp::SmtpError> {
    SmtpSender::from_override(ov)
}

/// Strip password from a `postgres://user:pass@host/db` URL so it is safe to log.
fn redact_db_url(url: &str) -> String {
    match (url.find("://"), url.find('@')) {
        (Some(scheme_end), Some(at)) if scheme_end + 3 < at => {
            let (prefix, rest) = url.split_at(scheme_end + 3);
            let (creds, host) = rest.split_at(at - (scheme_end + 3));
            let user = creds.split(':').next().unwrap_or("");
            if user.is_empty() {
                format!("{prefix}***{host}")
            } else {
                format!("{prefix}{user}:***{host}")
            }
        }
        _ => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_db_url_masks_password() {
        assert_eq!(
            redact_db_url("postgres://mailify:secret@localhost:5432/mailify"),
            "postgres://mailify:***@localhost:5432/mailify"
        );
    }

    #[test]
    fn redact_db_url_without_creds_is_unchanged() {
        assert_eq!(
            redact_db_url("postgres://localhost/mailify"),
            "postgres://localhost/mailify"
        );
    }
}
