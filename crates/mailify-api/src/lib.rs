pub mod error;
pub mod openapi;
pub mod routes;
pub mod state;

use std::sync::Arc;

use axum::{http::StatusCode, middleware, routing::get, Router};
use mailify_auth::{middleware::AuthLayer, require_jwt};
use tower_http::{
    cors::CorsLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub use state::AppState;

pub fn build_router(state: AppState) -> Router {
    let auth_layer = AuthLayer {
        issuer: state.jwt.clone(),
    };

    let protected = Router::new()
        .route(
            "/mail/send",
            axum::routing::post(routes::mail::send_registered),
        )
        .route(
            "/mail/send-custom",
            axum::routing::post(routes::mail::send_custom),
        )
        .route("/templates", get(routes::templates::list))
        .route(
            "/templates/:id/preview",
            get(routes::templates::preview_get).post(routes::templates::preview_post),
        )
        .route("/config", get(routes::config::get_config))
        .route_layer(middleware::from_fn_with_state(auth_layer, require_jwt));

    let public = Router::new()
        .route("/health", get(routes::health::health))
        .route(
            "/auth/token",
            axum::routing::post(routes::auth::issue_token),
        );

    let swagger =
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi::ApiDoc::openapi());

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(Arc::new(state))
        .merge(swagger)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(30),
        ))
}
