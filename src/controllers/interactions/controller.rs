use axum::{body::Bytes, extract::State, response::IntoResponse, Json};

use crate::{
    errors::AppError,
    services::interaction_type::InteractionTypeHelper,
    types::{ApiResponse, AppState},
};

pub async fn handle_interaction(State(state): State<AppState>, body: Bytes) -> ApiResponse {
    let raw_interaction: serde_json::Value = serde_json::from_slice(&body).map_err(|err| {
        tracing::error!("[PARSE ERROR] {err}");
        AppError::bad_request(err)
    })?;
    let interaction_type_str = &raw_interaction["type"];
    let interaction_type_u8 = interaction_type_str
        .to_string()
        .parse::<u8>()
        .map_err(AppError::bad_request)?;

    let interaction_type = interaction_type_u8.to_interaction_type();
    let response = interaction_type.build_response(state, body).await?;

    if cfg!(debug_assertions) {
        tracing::info!("[RESPONSE] {:#?}", response);
    }
    Ok(Json(response).into_response())
}
