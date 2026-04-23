use std::sync::Arc;

use mailify_api::{build_router, AppState};
use mailify_auth::{
    generate_bootstrap_key, generate_jwt_secret, print_bootstrap_banner, JwtIssuer,
};
use mailify_config::{AppConfig, LogFormat};
use mailify_queue::{worker::WorkerDeps, QueueRuntime};
use mailify_smtp::SmtpSender;
use mailify_templates::TemplateRegistry;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cfg = AppConfig::load()?;
    init_tracing(&cfg);

    info!(version = env!("CARGO_PKG_VERSION"), "starting mailify");

    maybe_bootstrap_auth(&mut cfg);

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

const DEFAULT_JWT_SECRET: &str = "CHANGE_ME_IN_PRODUCTION";
const BOOTSTRAP_KEY_ID: &str = "DEFAULT";

fn maybe_bootstrap_auth(cfg: &mut AppConfig) {
    if !cfg.auth.bootstrap || !cfg.auth.api_keys.is_empty() {
        return;
    }

    let key = match generate_bootstrap_key(BOOTSTRAP_KEY_ID) {
        Ok(k) => k,
        Err(e) => {
            error!(error = %e, "failed to generate bootstrap api key");
            return;
        }
    };

    let jwt_override = if cfg.auth.jwt_secret == DEFAULT_JWT_SECRET {
        let secret = generate_jwt_secret();
        cfg.auth.jwt_secret = secret.clone();
        Some(secret)
    } else {
        None
    };

    cfg.auth.api_keys.insert(key.id.clone(), key.hash.clone());

    print_bootstrap_banner(&key, jwt_override.as_deref());
}

fn init_tracing(cfg: &AppConfig) {
    // Precedence: RUST_LOG (standard) > cfg.observability.log_level > built-in fallback.
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&cfg.observability.log_level))
        .unwrap_or_else(|_| {
            EnvFilter::new(
                "info,mailify=debug,mailify_api=debug,mailify_queue=debug,tower_http=info",
            )
        });
    let registry = tracing_subscriber::registry().with(filter);
    match cfg.observability.log_format {
        LogFormat::Json => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        LogFormat::Pretty => registry
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_level(true)
                    .compact(),
            )
            .init(),
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
