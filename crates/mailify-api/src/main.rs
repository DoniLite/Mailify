use std::sync::Arc;

use mailify_api::{build_router, AppState};
use mailify_auth::JwtIssuer;
use mailify_config::{AppConfig, LogFormat};
use mailify_queue::{worker::WorkerDeps, QueueRuntime};
use mailify_smtp::SmtpSender;
use mailify_templates::TemplateRegistry;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = AppConfig::load()?;
    init_tracing(&cfg);

    info!(version = env!("CARGO_PKG_VERSION"), "starting mailify");

    info!(api_key_ids = ?cfg.auth.api_keys.keys().cloned().collect::<Vec<_>>(), "loaded auth api_key ids");

    let registry = Arc::new(TemplateRegistry::load_from_dir(
        &cfg.templates.path,
        cfg.i18n.clone(),
        cfg.templates.strict,
    )?);
    info!(count = registry.list_ids().len(), "templates loaded");

    let default_sender = Arc::new(SmtpSender::from_config(&cfg.smtp)?);

    let (queue_runtime, queue_handle) = QueueRuntime::init(
        &cfg,
        WorkerDeps {
            registry: registry.clone(),
            default_sender: default_sender.clone(),
            theme: cfg.theme.clone(),
        },
    )
    .await?;

    let jwt = Arc::new(JwtIssuer::new(
        cfg.auth.jwt_secret.clone(),
        cfg.auth.jwt_issuer.clone(),
        cfg.auth.jwt_ttl_secs,
    ));

    let state = AppState {
        cfg: Arc::new(cfg.clone()),
        registry,
        queue: queue_handle,
        jwt,
    };

    let shutdown = CancellationToken::new();
    let worker_token = shutdown.clone();
    let worker_task = tokio::spawn(async move {
        if let Err(e) = queue_runtime.run(worker_token).await {
            error!(error = %e, "queue runtime stopped with error");
        }
    });

    let app = build_router(state);

    let listener =
        tokio::net::TcpListener::bind((cfg.server.host.as_str(), cfg.server.port)).await?;
    info!(addr = %listener.local_addr()?, "http listening");

    let http_token = shutdown.clone();
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        shutdown_signal().await;
        http_token.cancel();
    });

    if let Err(e) = server.await {
        error!(error = %e, "http server error");
    }

    let _ = worker_task.await;
    info!("mailify stopped");
    Ok(())
}

fn init_tracing(cfg: &AppConfig) {
    let filter =
        EnvFilter::try_new(&cfg.observability.log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    let registry = tracing_subscriber::registry().with(filter);
    match cfg.observability.log_format {
        LogFormat::Json => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        LogFormat::Pretty => registry.with(tracing_subscriber::fmt::layer()).init(),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.ok();
    };
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        if let Ok(mut s) = signal(SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("shutdown signal received");
}
