use ed25519_dalek::VerifyingKey;

use crate::{env::Env, errors::AppError, types::AppState};

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        let env = Env::load()?;
        // discord verification key
        let public_key = match hex::decode(env.discord_public_key.clone()) {
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
        // lambda sdk
        let config = aws_config::load_from_env().await;

        Ok(Self {
            env,
            verification_key,
            request_client: reqwest::Client::new(),
            sdk_lambda_client: aws_sdk_lambda::Client::new(&config),
        })
    }
}
