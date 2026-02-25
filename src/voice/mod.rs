use crate::error::Result;
use crate::error::{StateError, TransportError};
use crate::gateway::{DispatchEvent, GatewayClient};
use crate::http::{AuthPolicy, Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::tri::Patch;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant, timeout};

const VOICE_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Clone)]
pub struct VoiceClient {
    http: HttpClient,
    gateway: GatewayClient,
    session: Arc<RwLock<Option<VoiceSession>>>,
}

impl VoiceClient {
    pub fn new(http: HttpClient, gateway: GatewayClient) -> Self {
        Self {
            http,
            gateway,
            session: Arc::new(RwLock::new(None)),
        }
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

    pub async fn join_voice(
        &self,
        guild_id: Snowflake,
        channel_id: Snowflake,
        self_mute: bool,
        self_deaf: bool,
    ) -> Result<VoiceSession> {
        let update = VoiceStateUpdate {
            guild_id,
            channel_id: Some(channel_id),
            self_mute,
            self_deaf,
            self_video: None,
            self_stream: None,
            connection_id: None,
        };
        self.update_voice_state(&update).await?;

        let session = self.wait_for_session(guild_id, channel_id).await?;
        *self.session.write().await = Some(session.clone());
        Ok(session)
    }

    pub async fn disconnect_voice(&self, guild_id: Snowflake) -> Result<()> {
        let update = VoiceStateUpdate {
            guild_id,
            channel_id: None,
            self_mute: false,
            self_deaf: false,
            self_video: None,
            self_stream: None,
            connection_id: None,
        };
        self.update_voice_state(&update).await?;
        *self.session.write().await = None;
        Ok(())
    }

    pub async fn session(&self) -> Option<VoiceSession> {
        self.session.read().await.clone()
    }

    async fn wait_for_session(
        &self,
        guild_id: Snowflake,
        channel_id: Snowflake,
    ) -> Result<VoiceSession> {
        let mut rx = self.gateway.subscribe_dispatch();
        let deadline = Instant::now() + VOICE_HANDSHAKE_TIMEOUT;

        let mut state: Option<VoiceStateHandshake> = None;
        let mut server: Option<VoiceServerHandshake> = None;

        while Instant::now() < deadline {
            let wait = deadline.saturating_duration_since(Instant::now());
            let recv = timeout(wait, rx.recv()).await;
            let envelope = match recv {
                Ok(Ok(v)) => v,
                Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => continue,
                Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => {
                    return Err(StateError::Closed.into());
                }
                Err(_) => return Err(TransportError::Timeout.into()),
            };

            match envelope.event {
                DispatchEvent::VoiceStateUpdate(payload) => {
                    if payload.guild_id == Some(guild_id) && payload.channel_id == Some(channel_id)
                    {
                        if let Some(session_id) =
                            payload.extra.get("session_id").and_then(Value::as_str)
                        {
                            state = Some(VoiceStateHandshake {
                                guild_id,
                                channel_id,
                                user_id: payload.user_id,
                                session_id: session_id.to_owned(),
                            });
                        }
                    }
                }
                DispatchEvent::VoiceServerUpdate(payload) => {
                    if payload.guild_id == guild_id {
                        if let Some(endpoint) = payload.endpoint {
                            server = Some(VoiceServerHandshake {
                                guild_id,
                                token: payload.token,
                                endpoint,
                            });
                        }
                    }
                }
                _ => {}
            }

            if let (Some(state), Some(server)) = (&state, &server) {
                return Ok(VoiceSession {
                    token: server.token.clone(),
                    endpoint: server.endpoint.clone(),
                    session_id: state.session_id.clone(),
                });
            }
        }

        Err(TransportError::Timeout.into())
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
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub region: Patch<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSession {
    pub token: String,
    pub endpoint: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceStateHandshake {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceServerHandshake {
    pub guild_id: Snowflake,
    pub token: String,
    pub endpoint: String,
}
