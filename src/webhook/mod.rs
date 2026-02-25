use crate::error::Result;
use crate::flags::MessageFlags;
use crate::http::{AuthPolicy, Endpoint, HttpClient, HttpMethod, MessageResponse, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct WebhookClient {
    http: HttpClient,
}

impl WebhookClient {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn list_channel(&self, channel_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel_id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn list_guild(&self, guild_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild_id}/webhooks")
            .compile(&QueryValues::new(), &[("guild_id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn create(
        &self,
        channel_id: Snowflake,
        body: &WebhookCreateRequest,
    ) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel_id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<WebhookCreateRequest, WebhookResponse>(&ep, Some(body))
            .await
    }

    pub async fn get(&self, webhook_id: Snowflake) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/webhooks/{webhook_id}").compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string())],
        )?;
        self.http
            .request_json::<(), WebhookResponse>(&ep, None)
            .await
    }

    pub async fn update(
        &self,
        webhook_id: Snowflake,
        body: &WebhookUpdateRequest,
    ) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/webhooks/{webhook_id}").compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string())],
        )?;
        self.http
            .request_json::<WebhookUpdateRequest, WebhookResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete(&self, webhook_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/webhooks/{webhook_id}").compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
    ) -> Result<WebhookTokenResponse> {
        let ep = Endpoint {
            method: HttpMethod::Get,
            route: "/webhooks/{webhook_id}/{token}",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string()), ("token", token)],
        )?;
        self.http
            .request_json::<(), WebhookTokenResponse>(&ep, None)
            .await
    }

    pub async fn update_with_token(
        &self,
        webhook_id: Snowflake,
        token: &str,
        body: &WebhookTokenUpdateRequest,
    ) -> Result<WebhookTokenResponse> {
        let ep = Endpoint {
            method: HttpMethod::Patch,
            route: "/webhooks/{webhook_id}/{token}",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string()), ("token", token)],
        )?;
        self.http
            .request_json::<WebhookTokenUpdateRequest, WebhookTokenResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_with_token(&self, webhook_id: Snowflake, token: &str) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Delete,
            route: "/webhooks/{webhook_id}/{token}",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("webhook_id", &webhook_id.to_string()), ("token", token)],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn execute(
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

        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/webhooks/{webhook_id}/{token}",
            auth: AuthPolicy::NoBot,
        }
        .compile(&q, &[("webhook_id", &webhook_id.to_string()), ("token", token)])?;
        self.http
            .request_json::<ExecuteWebhookRequest, Option<MessageResponse>>(&ep, Some(body))
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
        let mut q = QueryValues::new();
        q.insert_opt("wait", wait);
        q.insert_opt("thread_id", thread_id);

        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/webhooks/{webhook_id}/{token}/slack",
            auth: AuthPolicy::NoBot,
        }
        .compile(&q, &[("webhook_id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn execute_github(
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

        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/webhooks/{webhook_id}/{token}/github",
            auth: AuthPolicy::NoBot,
        }
        .compile(&q, &[("webhook_id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn execute_sentry(
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

        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/webhooks/{webhook_id}/{token}/sentry",
            auth: AuthPolicy::NoBot,
        }
        .compile(&q, &[("webhook_id", &webhook_id.to_string()), ("token", token)])?;
        self.http.request_unit(&ep, Some(body)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCreateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTokenUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub token: String,
    #[serde(default)]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTokenResponse {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub token: String,
    #[serde(default)]
    pub avatar: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackWebhookRequest {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub attachments: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubWebhook {
    pub sender: Value,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentryWebhook {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub actor: Option<Value>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub installation: Option<Value>,
}
