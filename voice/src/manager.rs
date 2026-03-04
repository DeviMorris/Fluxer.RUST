use dashmap::DashMap;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::connection::FluxerVoiceConnection;
use crate::error::VoiceError;

pub type GatewaySender = Arc<dyn Fn(Value) + Send + Sync>;

pub struct VoiceManager {
    active_connections: Arc<DashMap<String, Arc<FluxerVoiceConnection>>>,
    pending_connections: Arc<DashMap<String, Arc<Mutex<Option<Value>>>>>,
    gateway_sender: Arc<RwLock<Option<GatewaySender>>>,
}

impl Default for VoiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceManager {
    pub fn new() -> Self {
        Self {
            active_connections: Arc::new(DashMap::new()),
            pending_connections: Arc::new(DashMap::new()),
            gateway_sender: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_gateway_sender(&self, sender: GatewaySender) {
        let mut w = self.gateway_sender.write().await;
        *w = Some(sender);
    }

    pub async fn join(
        &self,
        guild_id: &str,
        channel_id: &str,
    ) -> Result<Arc<FluxerVoiceConnection>, VoiceError> {
        let channel_id_owned = channel_id.to_string();

        if let Some(conn) = self.active_connections.get(&channel_id_owned) {
            if conn.is_connected() {
                return Ok(conn.clone());
            } else {
                self.active_connections.remove(&channel_id_owned);
            }
        }

        let sender_opt = self.gateway_sender.read().await.clone();
        let sender = match sender_opt {
            Some(s) => s,
            None => return Err(VoiceError::GatewayUnavailable),
        };

        let slot: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
        self.pending_connections
            .insert(guild_id.to_string(), slot.clone());

        let payload = serde_json::json!({
            "op": 4,
            "d": {
                "guild_id": guild_id,
                "channel_id": channel_id,
                "self_mute": false,
                "self_deaf": false
            }
        });

        tracing::info!(
            "Sending opcode 4: guild_id={} channel_id={}",
            guild_id,
            channel_id
        );
        sender(payload);
        tracing::info!("Opcode 4 sent, polling for VOICE_SERVER_UPDATE...");

        let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_secs(60);
        let data = loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let guard = slot.lock().await;
            if let Some(val) = guard.clone() {
                drop(guard);
                self.pending_connections.remove(guild_id);
                break val;
            }
            drop(guard);

            if tokio::time::Instant::now() >= deadline {
                self.pending_connections.remove(guild_id);
                return Err(VoiceError::Timeout);
            }
        };

        tracing::info!("VOICE_SERVER_UPDATE received: {:?}", data);

        let token = data["token"]
            .as_str()
            .ok_or_else(|| VoiceError::ConnectionFailed("No token in response".into()))?;
        let endpoint = data["endpoint"]
            .as_str()
            .ok_or_else(|| VoiceError::ConnectionFailed("No endpoint in response".into()))?;

        let mut ep_str = endpoint.to_string();
        if !ep_str.starts_with("ws://") && !ep_str.starts_with("wss://") {
            ep_str = format!("wss://{}", ep_str);
        }

        let connection_id = data["connection_id"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        tracing::info!("Connecting to LiveKit: {}", ep_str);

        let conn = FluxerVoiceConnection::connect(
            &ep_str,
            token,
            guild_id.to_string(),
            channel_id_owned.clone(),
            connection_id,
        )
        .await?;

        self.active_connections
            .insert(channel_id_owned.clone(), conn.clone());
        Ok(conn)
    }

    pub async fn disconnect(&self, channel_id: &str) -> Result<(), VoiceError> {
        if let Some((_, conn)) = self.active_connections.remove(channel_id) {
            conn.disconnect().await?;

            let sender_opt = self.gateway_sender.read().await.clone();
            if let Some(sender) = sender_opt {
                let payload = serde_json::json!({
                    "op": 4,
                    "d": {
                        "guild_id": conn.guild_id.clone(),
                        "channel_id": serde_json::Value::Null,
                        "self_mute": false,
                        "self_deaf": false
                    }
                });
                sender(payload);
            }
        }
        Ok(())
    }

    pub async fn disconnect_guild(&self, guild_id: &str) -> Result<(), VoiceError> {
        let channel_ids: Vec<String> = self
            .active_connections
            .iter()
            .filter(|kv| kv.value().guild_id == guild_id)
            .map(|kv| kv.key().clone())
            .collect();
        for channel_id in channel_ids {
            self.disconnect(&channel_id).await?;
        }
        Ok(())
    }

    pub async fn stop_guild(&self, guild_id: &str) {
        for kv in self.active_connections.iter() {
            if kv.value().guild_id == guild_id {
                let _ = kv.value().stop().await;
            }
        }
    }

    pub async fn disconnect_all(&self) {
        let keys: Vec<String> = self
            .active_connections
            .iter()
            .map(|kv| kv.key().clone())
            .collect();
        for key in keys {
            let _ = self.disconnect(&key).await;
        }
    }

    pub fn get_connection(&self, channel_id: &str) -> Option<Arc<FluxerVoiceConnection>> {
        self.active_connections.get(channel_id).map(|c| c.clone())
    }

    pub fn is_connected(&self, channel_id: &str) -> bool {
        self.active_connections
            .get(channel_id)
            .is_some_and(|c| c.is_connected())
    }

    pub fn handle_voice_server_update(&self, data: Value) {
        tracing::info!(
            "handle_voice_server_update called, pending count: {}",
            self.pending_connections.len()
        );
        tracing::info!("data: {:?}", data);

        if let Some(g_id) = data["guild_id"].as_str() {
            if let Some(slot) = self.pending_connections.get(g_id) {
                let slot = slot.clone();
                let data = data.clone();
                tokio::spawn(async move {
                    let mut guard = slot.lock().await;
                    *guard = Some(data);
                });
                tracing::info!("Stored VOICE_SERVER_UPDATE for guild_id={}", g_id);
            } else {
                tracing::warn!("No pending connection for guild_id={}", g_id);
            }
        } else {
            tracing::warn!("VOICE_SERVER_UPDATE missing guild_id");
        }
    }
}
