use axum::{body::Bytes, extract::State, response::IntoResponse, Json};

use crate::{
    errors::AppError,
    types::{
        ApiResponse, AppState, Interaction, InteractionResponse, InteractionResponseData,
        InteractionType,
    },
};

pub async fn handle_interaction(State(state): State<AppState>, body: Bytes) -> ApiResponse {
    if cfg!(debug_assertions) {
        let raw_interaction: serde_json::Value =
            serde_json::from_slice(&body).map_err(AppError::bad_request)?;
        tracing::info!("{:#?}", raw_interaction);
    }
    let request_client = state.request_client;
    let interaction: Interaction = serde_json::from_slice(&body).map_err(AppError::bad_request)?;

    let response = match interaction.interaction_type {
        // type 1 is always discord ping
        InteractionType::Ping => InteractionResponse {
            response_type: 1,
            ..Default::default()
        },
        InteractionType::ApplicationCommand => {
            // handle slash commands
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                if let Err(err) = interaction.handle_slash_command(request_client).await {
                    tracing::error!("{:#?}", err);
                }
            });
            InteractionResponse {
                response_type: 5, // defer
                ..Default::default()
            }
        }
        InteractionType::Unhandled(_) => InteractionResponse {
            data: Some(InteractionResponseData {
                content: "Unsupported interaction".to_string(),
            }),
            ..Default::default()
        },
    };

    Ok(Json(response).into_response())
}
