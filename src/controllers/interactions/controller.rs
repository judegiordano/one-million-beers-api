use axum::{body::Bytes, extract::State, response::IntoResponse, Json};

use crate::types::{
    ApiResponse, AppState, Interaction, InteractionResponse, InteractionResponseData,
    InteractionType,
};

pub async fn handle_interaction(State(state): State<AppState>, body: Bytes) -> ApiResponse {
    let request_client = state.request_client;
    let interaction: Interaction = serde_json::from_slice(&body).unwrap_or_default();
    tracing::info!("[INTERACTION] {:?}", interaction);

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
                    tracing::error!(
                        "[SLASH COMMAND ERROR]: [COMMAND: {}] [USER: {}]  [ERROR: {:#?}]",
                        interaction.data.name,
                        interaction.member.user.id,
                        err
                    );
                }
            });
            InteractionResponse {
                response_type: 5, // defer
                data: Some(InteractionResponseData {
                    flags: Some(1 << 6), // 64 ephemeral message
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
        InteractionType::Unhandled(_) => InteractionResponse {
            data: Some(InteractionResponseData {
                content: "Unsupported interaction".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    if cfg!(debug_assertions) {
        tracing::info!("[RESPONSE] {:#?}", response);
    }
    Ok(Json(response).into_response())
}
