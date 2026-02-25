use super::messages::MessageResponse;
use crate::error::Result;
use crate::flags::MessageFlags;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct WebhooksApi {
    http: HttpClient,
}

impl WebhooksApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn list_channel_webhooks(
        &self,
        channel_id: Snowflake,
    ) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn create_channel_webhook(
        &self,
        channel_id: Snowflake,
        body: &WebhookCreateRequest,
    ) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<WebhookCreateRequest, WebhookResponse>(&ep, Some(body))
            .await
    }

    pub async fn list_guild_webhooks(&self, guild_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/webhooks").compile(
            &QueryValues::new(),
            &[("guild.id", &guild_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn get_webhook(&self, webhook_id: Snowflake) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/webhooks/{webhook.id}").compile(
            &QueryValues::new(),
            &[("webhook.id", &webhook_id.to_string())],
        )?;
        self.http
            .request_json::<(), WebhookResponse>(&ep, None)
            .await
    }

    pub async fn update_webhook(
        &self,
        webhook_id: Snowflake,
        body: &WebhookUpdateRequest,
    ) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/webhooks/{webhook.id}").compile(
            &QueryValues::new(),
            &[("webhook.id", &webhook_id.to_string())],
        )?;
        self.http
            .request_json::<WebhookUpdateRequest, WebhookResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_webhook(&self, webhook_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/webhooks/{webhook.id}").compile(
            &QueryValues::new(),
            &[("webhook.id", &webhook_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_webhook_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
    ) -> Result<WebhookTokenResponse> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/webhooks/{webhook.id}/{token}")
            .compile(
                &QueryValues::new(),
                &[("webhook.id", &webhook_id.to_string()), ("token", token)],
            )?;
        self.http
            .request_json::<(), WebhookTokenResponse>(&ep, None)
            .await
    }

    pub async fn update_webhook_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &WebhookTokenUpdateRequest,
    ) -> Result<WebhookTokenResponse> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Patch, "/webhooks/{webhook.id}/{token}")
            .compile(
                &QueryValues::new(),
                &[("webhook.id", &webhook_id.to_string()), ("token", token)],
            )?;
        self.http
            .request_json::<WebhookTokenUpdateRequest, WebhookTokenResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_webhook_with_token(&self, webhook_id: Snowflake, token: &str) -> Result<()> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Delete, "/webhooks/{webhook.id}/{token}")
            .compile(
                &QueryValues::new(),
                &[("webhook.id", &webhook_id.to_string()), ("token", token)],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn execute_webhook(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &ExecuteWebhookRequest,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<Option<MessageResponse>> {
        let mut q = QueryValues::new();
        q.insert_opt("wait", wait);
        q.insert_opt("thread_id", thread_id);

        let ep = Endpoint::new_no_bot_auth(HttpMethod::Post, "/webhooks/{webhook.id}/{token}")
            .compile(&q, &[("webhook.id", &webhook_id.to_string()), ("token", token)])?;
        self.http
            .request_json::<ExecuteWebhookRequest, Option<MessageResponse>>(&ep, Some(body))
            .await
    }

    pub async fn execute_webhook_github(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &GitHubWebhook,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        let mut q = QueryValues::new();
        q.insert_opt("wait", wait);
        q.insert_opt("thread_id", thread_id);

        let ep =
            Endpoint::new_no_bot_auth(HttpMethod::Post, "/webhooks/{webhook.id}/{token}/github")
                .compile(&q, &[("webhook.id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn execute_webhook_sentry(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &SentryWebhook,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        let mut q = QueryValues::new();
        q.insert_opt("wait", wait);
        q.insert_opt("thread_id", thread_id);

        let ep =
            Endpoint::new_no_bot_auth(HttpMethod::Post, "/webhooks/{webhook.id}/{token}/sentry")
                .compile(&q, &[("webhook.id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn execute_webhook_slack(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &SlackWebhookRequest,
        wait: Option<bool>,
        thread_id: Option<Snowflake>,
    ) -> Result<()> {
        let mut q = QueryValues::new();
        q.insert_opt("wait", wait);
        q.insert_opt("thread_id", thread_id);

        let ep =
            Endpoint::new_no_bot_auth(HttpMethod::Post, "/webhooks/{webhook.id}/{token}/slack")
                .compile(&q, &[("webhook.id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCreateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookTokenUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTokenResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlackWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubWebhook {
    pub sender: Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentryWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<Value>,
}
