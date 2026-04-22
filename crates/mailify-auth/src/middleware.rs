use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::jwt::{Claims, JwtIssuer};

#[derive(Clone)]
pub struct AuthLayer {
    pub issuer: Arc<JwtIssuer>,
}

/// Axum middleware: extract `Authorization: Bearer <jwt>`, verify, inject `Claims` extension.
pub async fn require_jwt(
    State(layer): State<AuthLayer>,
    mut req: Request,
    next: Next,
) -> Response {
    let Some(header) = req.headers().get(axum::http::header::AUTHORIZATION) else {
        return unauthorized("missing Authorization header");
    };
    let Ok(header) = header.to_str() else {
        return unauthorized("invalid Authorization header");
    };
    let Some(token) = header.strip_prefix("Bearer ") else {
        return unauthorized("expected Bearer scheme");
    };
    match layer.issuer.verify(token) {
        Ok(claims) => {
            req.extensions_mut().insert::<Claims>(claims);
            next.run(req).await
        }
        Err(e) => {
            tracing::debug!(error = %e, "jwt verification failed");
            unauthorized("invalid token")
        }
    }
}

fn unauthorized(msg: &'static str) -> Response {
    (StatusCode::UNAUTHORIZED, msg).into_response()
}
