use one_million_beers_api::{env::Env, errors::AppError, logger};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct SlashCommand {
    pub name: String,
    pub description: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    logger::init()?;
    let Env {
        discord_client_id,
        discord_client_secret,
        ..
    } = Env::load()?;

    let url = format!("https://discord.com/api/v10/applications/{discord_client_id}/commands");

    let cmd_buffer = std::fs::read("./commands.json").map_err(AppError::not_found)?;
    let data = serde_json::from_slice(&cmd_buffer).map_err(AppError::internal_server_error)?;

    let client = reqwest::Client::new();

    let res = client
        .put(&url)
        .header("Authorization", format!("Bot {discord_client_secret}"))
        .json::<Vec<SlashCommand>>(&data)
        .send()
        .await
        .map_err(AppError::internal_server_error)?;

    let status = res.status();
    let json = res
        .json::<serde_json::Value>()
        .await
        .map_err(AppError::internal_server_error)?;

    tracing::info!("{status:?}: {json:#?}");
    Ok(())
}
