use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::types::ApiResponse;

pub async fn handle_interaction() -> ApiResponse {
    Ok(Json(json!({ "ok": true })).into_response())
}
