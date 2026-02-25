use crate::error::Result;
use crate::http::{HttpClient, MessageResponse, WebhooksApi};
use crate::id::Snowflake;

#[derive(Debug, Clone)]
pub struct WebhookClient {
    api: WebhooksApi,
}

impl WebhookClient {
    pub fn new(http: HttpClient) -> Self {
        Self {
            api: WebhooksApi::new(http),
        }
    }

    pub async fn list_channel(&self, channel_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        self.api.list_channel_webhooks(channel_id).await
    }

    pub async fn list_guild(&self, guild_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        self.api.list_guild_webhooks(guild_id).await
    }

    pub async fn create(
        &self,
        channel_id: Snowflake,
        body: &WebhookCreateRequest,
    ) -> Result<WebhookResponse> {
        self.api.create_channel_webhook(channel_id, body).await
    }

    pub async fn get(&self, webhook_id: Snowflake) -> Result<WebhookResponse> {
        self.api.get_webhook(webhook_id).await
    }

    pub async fn update(
        &self,
        webhook_id: Snowflake,
        body: &WebhookUpdateRequest,
    ) -> Result<WebhookResponse> {
        self.api.update_webhook(webhook_id, body).await
    }

    pub async fn delete(&self, webhook_id: Snowflake) -> Result<()> {
        self.api.delete_webhook(webhook_id).await
    }

    pub async fn get_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
    ) -> Result<WebhookTokenResponse> {
        self.api.get_webhook_with_token(webhook_id, token).await
    }

    pub async fn update_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &WebhookTokenUpdateRequest,
    ) -> Result<WebhookTokenResponse> {
        self.api
            .update_webhook_with_token(webhook_id, token, body)
            .await
    }

    pub async fn delete_with_token(&self, webhook_id: Snowflake, token: &str) -> Result<()> {
        self.api.delete_webhook_with_token(webhook_id, token).await
    }

    pub async fn execute(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &ExecuteWebhookRequest,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<Option<MessageResponse>> {
        self.api
            .execute_webhook(webhook_id, token, body, wait, thread_id)
            .await
    }

    pub async fn execute_slack(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &SlackWebhookRequest,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        self.api
            .execute_webhook_slack(webhook_id, token, body, wait, thread_id)
            .await
    }

    pub async fn execute_github(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &GitHubWebhook,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        self.api
            .execute_webhook_github(webhook_id, token, body, wait, thread_id)
            .await
    }

    pub async fn execute_sentry(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &SentryWebhook,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        self.api
            .execute_webhook_sentry(webhook_id, token, body, wait, thread_id)
            .await
    }
}

pub use crate::http::ExecuteWebhookRequest;
pub use crate::http::GitHubWebhook;
pub use crate::http::SentryWebhook;
pub use crate::http::SlackWebhookRequest;
pub use crate::http::WebhookCreateRequest;
pub use crate::http::WebhookResponse;
pub use crate::http::WebhookTokenResponse;
pub use crate::http::WebhookTokenUpdateRequest;
pub use crate::http::WebhookUpdateRequest;
