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
            "https://discord.com/api/v10/webhooks/{}/{}/messages/@original",
            application_id, token
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
            "ping" => self._ping().await?,
            "beer-down" => self._beer_down().await?,
            _ => self._unhandled().await?,
        };
        let _ = self.send_followup_message(request_client, content).await;
        Ok(())
    }

    async fn _unhandled(&self) -> Result<String, AppError> {
        Ok("WIP command".to_string())
    }

    async fn _ping(&self) -> Result<String, AppError> {
        Ok("pong".to_string())
    }

    async fn _beer_down(&self) -> Result<String, AppError> {
        let server_id = &self.guild_id;
        let user_id = &self.member.user.id;
        let (user, server) = try_join!(
            User::increment_user_total(server_id, user_id),
            Server::increment_server_total(server_id)
        )
        .map_err(AppError::bad_request)?;
        Ok(format!(
            "{} beer(s) down! Server is at {} 🍻.",
            user.server_total, server.server_total
        ))
    }
}
