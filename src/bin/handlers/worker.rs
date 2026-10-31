use lambda_http::LambdaEvent;
use one_million_beers_api::{env::Env, types::Interaction};
use tower::service_fn;

async fn handler(event: LambdaEvent<Interaction>) -> Result<(), lambda_runtime::Error> {
    let request_client = reqwest::Client::new();
    let (interaction, _) = event.into_parts();

    if let Err(err) = interaction.handle_slash_command(request_client).await {
        tracing::error!(
            "[SLASH COMMAND ERROR]: [COMMAND: {}] [USER: {}]  [ERROR: {:#?}]",
            interaction.data.name,
            interaction.member.user.id,
            err
        );
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_runtime::Error> {
    let _ = Env::load()?;
    lambda_runtime::run(service_fn(handler)).await
}
