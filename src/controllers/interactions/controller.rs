use axum::{body::Bytes, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    errors::AppError,
    types::{ApiResponse, Interaction},
};

pub async fn handle_interaction(body: Bytes) -> ApiResponse {
    let interaction: Interaction = serde_json::from_slice(&body).map_err(AppError::bad_request)?;
    tracing::info!("{:#?}", interaction);

    let response = match interaction.interaction_type {
        // type 1 is always discord ping
        1 => json!({ "type": 1 }),
        2 => {
            let name = interaction.data.name.as_str();
            json!({
                "type": 4,
                "data": { "content": format!("You ran /{name}") }
            })
        }

        _ => json!({ "type": 4, "data": { "content": "Unsupported interaction" } }),
    };

    Ok(Json(response).into_response())
}
