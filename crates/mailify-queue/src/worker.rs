use std::sync::Arc;

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_sql::postgres::{PgListen, PostgresStorage};
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
    #[error("worker error: {0}")]
    Worker(String),
}

/// Handle used by HTTP routes to enqueue jobs.
#[derive(Clone)]
pub struct QueueHandle {
    storage: PostgresStorage<MailJob>,
}

impl QueueHandle {
    pub async fn push(&mut self, job: MailJob) -> Result<uuid::Uuid, QueueError> {
        let id = job.id;
        self.storage
            .push(job)
            .await
            .map_err(|e| QueueError::Push(e.to_string()))?;
        Ok(id)
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
    pub async fn init(app_cfg: &AppConfig, deps: WorkerDeps) -> Result<(Self, QueueHandle), QueueError> {
        let pool = PgPoolOptions::new()
            .max_connections(app_cfg.database.max_connections)
            .min_connections(app_cfg.database.min_connections)
            .connect(&app_cfg.database.url)
            .await?;

        PostgresStorage::setup(&pool)
            .await
            .map_err(|e| QueueError::Migrate(e.to_string()))?;

        let storage = PostgresStorage::new(pool.clone());
        let handle = QueueHandle { storage: storage.clone() };

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
    pub async fn run(self, shutdown: tokio_util::sync::CancellationToken) -> Result<(), QueueError> {
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
        MailJobKind::Custom { html, subject, text } => renderer
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

    sender.send(&envelope, &rendered).await.map_err(|e| e.to_string())?;
    info!(job_id = %job.id, "mail sent");
    Ok(())
}

fn sender_from_override(ov: &SmtpOverride) -> Result<SmtpSender, mailify_smtp::SmtpError> {
    SmtpSender::from_override(ov)
}
