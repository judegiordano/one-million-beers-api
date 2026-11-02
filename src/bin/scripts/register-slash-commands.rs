use one_million_beers_api::{env::Env, errors::AppError, logger, services::command::SlashCommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct SlashCommandPayload {
    pub name: String,
    pub description: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    logger::init()?;
    let Env {
        discord_client_id,
        discord_token,
        ..
    } = Env::load()?;

    let url = format!("https://discord.com/api/v10/applications/{discord_client_id}/commands");

    let commands = SlashCommand::variants();

    let slash_commands = commands
        .iter()
        .map(|cmd| SlashCommandPayload {
            name: cmd.name(),
            description: cmd.description(),
        })
        .collect::<Vec<_>>();

    let client = reqwest::Client::new();

    let res = client
        .put(&url)
        .header("Authorization", format!("Bot {discord_token}"))
        .json::<Vec<SlashCommandPayload>>(&slash_commands)
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
