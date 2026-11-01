use aws_sdk_lambda::{primitives::Blob, types::InvocationType};
use axum::body::Bytes;

use crate::{
    errors::AppError,
    types::{
        AppState, Interaction, InteractionResponse, InteractionResponseData, InteractionType,
        MessageComponentInteraction,
    },
};

pub trait InteractionTypeHelper {
    fn to_interaction_type(&self) -> InteractionType;
}

impl InteractionTypeHelper for u8 {
    fn to_interaction_type(&self) -> InteractionType {
        match self {
            1 => InteractionType::Ping,
            2 => InteractionType::ApplicationCommand,
            3 => InteractionType::MessageComponent,
            x => InteractionType::Unhandled(*x),
        }
    }
}

impl From<u8> for InteractionType {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Ping,
            2 => Self::ApplicationCommand,
            3 => Self::MessageComponent,
            n => Self::Unhandled(n),
        }
    }
}

impl InteractionType {
    pub async fn build_response(
        &self,
        state: AppState,
        body: Bytes,
    ) -> Result<InteractionResponse, AppError> {
        let response = match self {
            InteractionType::Ping => Self::ping(),
            InteractionType::ApplicationCommand => Self::application_command(state, body).await?,
            InteractionType::MessageComponent => Self::message_component(body)?,
            InteractionType::Unhandled(_) => Self::unhandled(),
        };
        Ok(response)
    }

    fn ping() -> InteractionResponse {
        InteractionResponse {
            response_type: 1,
            data: None,
        }
    }

    fn unhandled() -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionResponseData {
                content: "Unsupported interaction".to_string(),
                flags: Some(1 << 6),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    async fn application_command(
        state: AppState,
        body: Bytes,
    ) -> Result<InteractionResponse, AppError> {
        let interaction: Interaction = serde_json::from_slice(&body).map_err(|err| {
            tracing::error!("[PARSE ERROR] {err}");
            AppError::bad_request(err)
        })?;
        tracing::info!("[INTERACTION] {:?}", interaction);
        let flags = interaction.build_flags();

        if cfg!(debug_assertions) {
            // locally, spawn task
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                if let Err(err) = interaction.handle_slash_command(state.request_client).await {
                    tracing::error!(
                        "[SLASH COMMAND ERROR]: [COMMAND: {}] [USER: {}]  [ERROR: {:#?}]",
                        interaction.data.name,
                        interaction.member.user.id,
                        err
                    );
                }
            });
        } else {
            // in release, invoke worker lambda
            let function = &state.env.worker_function_name;
            let lambda = state.sdk_lambda_client.invoke();
            if let Err(err) = lambda
                .function_name(function)
                .invocation_type(InvocationType::Event)
                .payload(Blob::new(body.to_vec()))
                .send()
                .await
            {
                tracing::error!("[WORKER FUNCTION ERROR] {:?}", err);
                return Ok(InteractionResponse {
                    response_type: 4,
                    data: Some(InteractionResponseData {
                        content: "Something went wrong, try again.".to_string(),
                        flags,
                    }),
                });
            }
        }

        Ok(InteractionResponse {
            response_type: 5, // defer
            data: Some(InteractionResponseData {
                flags,
                ..Default::default()
            }),
        })
    }

    fn message_component(body: Bytes) -> Result<InteractionResponse, AppError> {
        let interaction: MessageComponentInteraction =
            serde_json::from_slice(&body).map_err(|err| {
                tracing::error!("[PARSE ERROR] {err}");
                AppError::bad_request(err)
            })?;

        if let Some(custom_id) = interaction.data.custom_id {
            match custom_id.as_str() {
                "share_message" => {
                    let content = format!(
                        "<@{}>\n{}",
                        interaction.member.user.id,
                        interaction.message.content.unwrap_or_default()
                    );
                    return Ok(InteractionResponse {
                        response_type: 4, //  public message no ephemeral flag
                        data: Some(InteractionResponseData {
                            content,
                            ..Default::default()
                        }),
                    });
                }
                _ => {
                    return Ok(InteractionResponse {
                        response_type: 4,
                        data: Some(InteractionResponseData {
                            content: "Message response interaction unhandled".to_string(),
                            flags: Some(1 << 6),
                        }),
                    })
                }
            }
        }

        Ok(InteractionResponse {
            response_type: 4,
            data: Some(InteractionResponseData {
                content: "Unsupported message response interaction".to_string(),
                flags: Some(1 << 6),
            }),
        })
    }
}
