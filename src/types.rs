use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use crate::{env::Env, errors::AppError};

pub type ApiResponse = Result<axum::response::Response, AppError>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub env: Env,
    pub verification_key: VerifyingKey,
    pub request_client: reqwest::Client,
    pub sdk_lambda_client: aws_sdk_lambda::Client,
}

#[derive(Debug, Deserialize, Default)]
pub struct InteractionChannel {
    pub id: String,
    pub name: String,
    pub guild_id: String,
    pub last_message_id: String,
    #[serde(rename = "type")]
    pub interaction_type: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct User {
    pub id: String,
    pub global_name: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct GuildMember {
    pub roles: Vec<String>,
    pub user: User,
}

#[derive(Debug, Deserialize, Default)]
pub struct InteractionData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub interaction_type: u64,
    pub component_type: Option<u64>,
    pub custom_id: Option<String>,
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Deserialize, Default)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub guild_id: String,
    pub channel_id: String,
    pub channel: InteractionChannel,
    pub member: GuildMember,
    pub token: String,
    pub data: InteractionData,
}

#[derive(Debug, Deserialize, Default)]
pub struct MessageComponentInteractionData {
    pub custom_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct MessageComponentInteractionMessage {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct MessageComponentInteraction {
    #[serde(rename = "type")]
    pub interaction_type: u64,
    pub data: MessageComponentInteractionData,
    pub member: GuildMember,
    pub message: MessageComponentInteractionMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(from = "u8")]
pub enum InteractionType {
    #[default]
    Ping,
    ApplicationCommand,
    MessageComponent,
    Unhandled(u8),
}

#[derive(Debug, Serialize, Default)]
pub struct InteractionResponseData {
    pub content: String,
    pub flags: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct InteractionResponse {
    // 1 -> ping
    // 4 -> slash command
    // 5 -> deferred
    #[serde(rename = "type")]
    pub response_type: u64,
    pub data: Option<InteractionResponseData>,
}

impl Default for InteractionResponse {
    fn default() -> Self {
        Self {
            response_type: 4, // default for non PONG response
            data: Option::default(),
        }
    }
}
