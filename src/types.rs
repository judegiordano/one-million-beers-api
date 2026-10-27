use ed25519_dalek::VerifyingKey;
use serde::Deserialize;

use crate::{env::Env, errors::AppError};

pub type ApiResponse = Result<axum::response::Response, AppError>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub verification_key: VerifyingKey,
}

impl AppState {
    pub fn new() -> Result<Self, AppError> {
        let Env {
            discord_public_key, ..
        } = Env::load()?;
        let public_key = match hex::decode(discord_public_key) {
            Ok(key) => key,
            Err(err) => return Err(AppError::InternalServerError(err.to_string())),
        };
        let public_key_bytes: [u8; 32] = match public_key.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(AppError::InternalServerError(
                    "unable to parse key into bytes".to_string(),
                ))
            }
        };
        let verification_key = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(key) => key,
            Err(err) => return Err(AppError::InternalServerError(err.to_string())),
        };
        Ok(Self { verification_key })
    }
}

#[derive(Debug, Deserialize)]
pub struct InteractionChannel {
    pub id: String,
    pub name: String,
    pub guild_id: String,
    pub last_message_id: String,
    #[serde(rename = "type")]
    pub interaction_type: u64,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub global_name: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct GuildMember {
    pub roles: Vec<String>,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub interaction_type: u64,
}

#[derive(Debug, Deserialize)]
pub struct Interaction {
    pub id: String,
    #[serde(rename = "type")]
    pub interaction_type: u64,
    pub guild_id: String,
    pub channel_id: String,
    pub channel: InteractionChannel,
    pub member: GuildMember,
    pub token: String,
    pub data: InteractionData,
}

pub enum InteractionType {
    PING = 1,
}
