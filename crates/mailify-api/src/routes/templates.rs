use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct TemplateInfo {
    pub id: String,
    pub locales: Vec<String>,
}

pub async fn list(State(state): State<Arc<AppState>>) -> Json<Vec<TemplateInfo>> {
    let mut grouped: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for key in state.registry.list() {
        grouped.entry(key.id).or_default().push(key.locale);
    }
    let out: Vec<TemplateInfo> = grouped
        .into_iter()
        .map(|(id, mut locales)| {
            locales.sort();
            locales.dedup();
            TemplateInfo { id, locales }
        })
        .collect();
    Json(out)
}
