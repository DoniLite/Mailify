use std::sync::Arc;

use axum::{extract::State, Json};
use mailify_auth::verify_api_key;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{error::ApiError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct IssueTokenRequest {
    pub api_key_id: String,
    pub api_key: String,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct IssueTokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
}

pub async fn issue_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<IssueTokenRequest>,
) -> Result<Json<IssueTokenResponse>, ApiError> {
    let Some(stored) = state.cfg.auth.api_keys.get(&body.api_key_id) else {
        warn!(id = %body.api_key_id, "unknown api_key_id");
        return Err(ApiError::Unauthorized);
    };

    let ok = verify_api_key(&body.api_key, stored)
        .map_err(|e| ApiError::Internal(format!("api key verify failed: {e}")))?;
    if !ok {
        warn!(id = %body.api_key_id, "api key mismatch");
        return Err(ApiError::Unauthorized);
    }

    let token = state
        .jwt
        .issue(&body.api_key_id, body.scopes)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(IssueTokenResponse {
        access_token: token,
        token_type: "Bearer",
        expires_in: state.cfg.auth.jwt_ttl_secs,
    }))
}
