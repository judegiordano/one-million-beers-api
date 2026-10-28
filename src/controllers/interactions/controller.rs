use axum::{body::Bytes, response::IntoResponse, Json};

use crate::{
    errors::AppError,
    types::{
        ApiResponse, Interaction, InteractionResponse, InteractionResponseData, InteractionType,
    },
};

pub async fn handle_interaction(body: Bytes) -> ApiResponse {
    let interaction: Interaction = serde_json::from_slice(&body).map_err(AppError::bad_request)?;
    tracing::info!("{:#?}", interaction);

    let response = match interaction.interaction_type {
        // type 1 is always discord ping
        InteractionType::Ping => InteractionResponse {
            response_type: 1,
            ..Default::default()
        },
        InteractionType::ApplicationCommand => {
            let name = interaction.data.name.as_str();
            // handle slash commands
            let content = match name {
                "ping" => "pong".to_string(),
                _ => format!("/{name} is a WIP"),
            };
            InteractionResponse {
                data: Some(InteractionResponseData { content }),
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
