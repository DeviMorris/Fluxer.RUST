use crate::error::Result;
use crate::gateway::GatewayClient;
use crate::http::{AuthPolicy, Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct VoiceClient {
    http: HttpClient,
    gateway: GatewayClient,
}

impl VoiceClient {
    pub fn new(http: HttpClient, gateway: GatewayClient) -> Self {
        Self { http, gateway }
    }

    pub fn gateway(&self) -> &GatewayClient {
        &self.gateway
    }

    pub async fn get_call_eligibility(
        &self,
        bearer_token: &str,
        channel_id: Snowflake,
    ) -> Result<CallEligibilityResponse> {
        let ep = Endpoint {
            method: HttpMethod::Get,
            route: "/channels/{channel_id}/call",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;

        self.http
            .request_json_with_auth::<(), CallEligibilityResponse>(
                &ep,
                None,
                Some(&format!("Bearer {bearer_token}")),
            )
            .await
    }

    pub async fn update_call_region(
        &self,
        bearer_token: &str,
        channel_id: Snowflake,
        body: &CallUpdateBody,
    ) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Patch,
            route: "/channels/{channel_id}/call",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;

        self.http
            .request_unit_with_auth(&ep, Some(body), Some(&format!("Bearer {bearer_token}")))
            .await
    }

    pub async fn end_call(&self, bearer_token: &str, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/channels/{channel_id}/call/end",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;

        self.http
            .request_unit_with_auth::<()>(&ep, None, Some(&format!("Bearer {bearer_token}")))
            .await
    }

    pub async fn ring(
        &self,
        bearer_token: &str,
        channel_id: Snowflake,
        body: &CallRingBody,
    ) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/channels/{channel_id}/call/ring",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;

        self.http
            .request_unit_with_auth(&ep, Some(body), Some(&format!("Bearer {bearer_token}")))
            .await
    }

    pub async fn stop_ringing(
        &self,
        bearer_token: &str,
        channel_id: Snowflake,
        body: &CallRingBody,
    ) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/channels/{channel_id}/call/stop-ringing",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;

        self.http
            .request_unit_with_auth(&ep, Some(body), Some(&format!("Bearer {bearer_token}")))
            .await
    }

    pub async fn update_voice_state(&self, payload: &VoiceStateUpdate) -> Result<()> {
        self.gateway
            .send(4, serde_json::to_value(payload).map_err(serde_to_protocol)?)
            .await
    }
}

fn serde_to_protocol(err: serde_json::Error) -> crate::error::Error {
    crate::error::Error::Protocol(crate::error::ProtocolError::Json(err))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEligibilityResponse {
    pub ringable: bool,
    pub silent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallUpdateBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRingBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<Snowflake>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceStateUpdate {
    pub guild_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    pub self_mute: bool,
    pub self_deaf: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_video: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_stream: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}
