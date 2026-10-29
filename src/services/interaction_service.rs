use serde_json::json;
use tokio::try_join;

use crate::{
    errors::AppError,
    models::{server::Server, user::User},
    types::Interaction,
};

impl Interaction {
    pub async fn send_followup_message(
        &self,
        request_client: reqwest::Client,
        content: String,
    ) -> Result<(), reqwest::Error> {
        let application_id = &self.application_id;
        let token = &self.token;

        let followup_url = format!(
            "https://discord.com/api/v10/webhooks/{application_id}/{token}/messages/@original",
        );

        let body = json!({ "content": content });

        let req = request_client.patch(followup_url).json(&body);
        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            tracing::error!(
                "followup failed: {status} {text}: [USER]: {} [GUILD]: {}",
                self.member.user.id,
                self.guild_id,
            );
        }
        Ok(())
    }

    pub async fn handle_slash_command(
        &self,
        request_client: reqwest::Client,
    ) -> Result<(), AppError> {
        let content = match self.data.name.as_str() {
            "ping" => Self::ping(),
            "beer-down" => self.beer_down().await?,
            "beer-count" => self.beer_count().await?,
            "beer-tab" => self.beer_tab().await?,
            _ => Self::unhandled(),
        };
        let _ = self.send_followup_message(request_client, content).await;
        Ok(())
    }

    fn unhandled() -> String {
        "WIP command".to_string()
    }

    fn ping() -> String {
        "pong".to_string()
    }

    async fn beer_down(&self) -> Result<String, AppError> {
        let server_id = &self.guild_id;
        let user_id = &self.member.user.id;
        let (_, server) = try_join!(
            User::increment_user_total(server_id, user_id),
            Server::increment_server_total(server_id)
        )
        .map_err(AppError::bad_request)?;
        Ok(format!(
            "Beer down! Server is at {} 🍻.",
            server.server_total
        ))
    }

    async fn beer_count(&self) -> Result<String, AppError> {
        let server_id = &self.guild_id;
        let count = Server::beer_count(server_id)
            .await
            .map_err(AppError::bad_request)?;
        Ok(format!("Server is at {count} beers 🍻.",))
    }

    async fn beer_tab(&self) -> Result<String, AppError> {
        let server_id = &self.guild_id;
        let user_id = &self.member.user.id;
        let count = User::beer_count(server_id, user_id)
            .await
            .map_err(AppError::bad_request)?;
        Ok(format!("You have downed {count} beers 🍻.",))
    }
}
