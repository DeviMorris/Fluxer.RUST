use std::collections::HashMap;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::connection::{VoiceConnection, VoiceEvent};

/// Voice state: guild_id → user_id → channel_id.
pub type VoiceStateMap = HashMap<String, HashMap<String, Option<String>>>;

/// Options for VoiceManager.
#[derive(Debug, Clone, Default)]
pub struct VoiceManagerOptions {
    pub shard_id: u32,
}

/// Manages voice connections across guilds and channels.
///
/// Tracks voice states from gateway, handles voice server/state updates,
/// creates and manages `VoiceConnection` instances.
///
/// # Examples
/// ```rust,ignore
/// let mut vm = VoiceManager::new(VoiceManagerOptions::default());
///
/// // Feed gateway events
/// vm.handle_voice_server_update(&server_data);
/// vm.handle_voice_state_update(&state_data);
///
/// // Join
/// let conn = vm.join("guild_id", "channel_id", "bot_user_id").await?;
/// ```
pub struct VoiceManager {
    #[allow(dead_code)]
    options: VoiceManagerOptions,
    connections: HashMap<String, VoiceConnection>,
    connection_ids: HashMap<String, String>,
    pub voice_states: VoiceStateMap,
    pending: HashMap<
        String,
        PendingVoice,
    >,
}

struct PendingVoice {
    guild_id: String,
    channel_id: String,
    user_id: String,
    server: Option<Value>,
    state: Option<Value>,
    tx: mpsc::UnboundedSender<Result<(), String>>,
}

impl VoiceManager {
    pub fn new(options: VoiceManagerOptions) -> Self {
        Self {
            options,
            connections: HashMap::new(),
            connection_ids: HashMap::new(),
            voice_states: HashMap::new(),
            pending: HashMap::new(),
        }
    }

    /// Get the voice channel ID for a user in a guild.
    pub fn get_voice_channel_id(&self, guild_id: &str, user_id: &str) -> Option<&str> {
        self.voice_states
            .get(guild_id)?
            .get(user_id)?
            .as_deref()
    }

    /// List user IDs currently in a specific voice channel.
    pub fn list_participants(&self, guild_id: &str, channel_id: &str) -> Vec<String> {
        let Some(guild_map) = self.voice_states.get(guild_id) else {
            return Vec::new();
        };
        guild_map
            .iter()
            .filter_map(|(user_id, ch)| {
                if ch.as_deref() == Some(channel_id) {
                    Some(user_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Handle gateway `VOICE_STATE_UPDATE`.
    pub fn handle_voice_state_update(&mut self, data: &Value) {
        let guild_id = data
            .get("guild_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let user_id = data
            .get("user_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let channel_id = data
            .get("channel_id")
            .and_then(|v| v.as_str())
            .map(String::from);

        if guild_id.is_empty() {
            return;
        }

        self.voice_states
            .entry(guild_id.clone())
            .or_default()
            .insert(user_id, channel_id);

        if let Some(connection_id) = data.get("connection_id").and_then(|v| v.as_str()) {
            if let Some(ch) = data.get("channel_id").and_then(|v| v.as_str()) {
                self.connection_ids
                    .insert(ch.to_string(), connection_id.to_string());
            }
        }

        let channel_key = data
            .get("channel_id")
            .and_then(|v| v.as_str())
            .unwrap_or(&guild_id);

        if let Some(pending) = self.pending.get_mut(channel_key) {
            pending.state = Some(data.clone());
            self.try_complete_pending(channel_key);
        }
    }

    /// Handle gateway `VOICE_SERVER_UPDATE`.
    pub fn handle_voice_server_update(&mut self, data: &Value) {
        let guild_id = data
            .get("guild_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let mut matching_key = None;
        for (key, p) in &self.pending {
            if p.guild_id == guild_id {
                matching_key = Some(key.clone());
                break;
            }
        }

        if let Some(key) = matching_key {
            if let Some(pending) = self.pending.get_mut(&key) {
                pending.server = Some(data.clone());
            }
            self.try_complete_pending(&key);
        }
    }

    /// Handle initial `VOICE_STATES_SYNC` bulk data from READY.
    pub fn handle_voice_states_sync(
        &mut self,
        guild_id: &str,
        states: &[Value],
    ) {
        let guild_map = self.voice_states.entry(guild_id.to_string()).or_default();
        for vs in states {
            let user_id = vs
                .get("user_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let channel_id = vs.get("channel_id").and_then(|v| v.as_str()).map(String::from);
            guild_map.insert(user_id, channel_id);
        }
    }

    fn try_complete_pending(&mut self, channel_key: &str) {
        let has_both = self
            .pending
            .get(channel_key)
            .map(|p| p.server.is_some() && p.state.is_some())
            .unwrap_or(false);

        if !has_both {
            return;
        }

        let pending = self.pending.remove(channel_key).unwrap();
        let (event_tx, _event_rx) = mpsc::unbounded_channel::<VoiceEvent>();
        let mut conn = VoiceConnection::new(
            &pending.guild_id,
            &pending.channel_id,
            &pending.user_id,
            event_tx,
        );

        let server = pending.server.unwrap();
        let state = pending.state.unwrap();
        let result_tx = pending.tx;

        tokio::spawn(async move {
            match conn.connect(&server, &state).await {
                Ok(()) => {
                    let _ = result_tx.send(Ok(()));
                }
                Err(e) => {
                    let _ = result_tx.send(Err(e));
                }
            }
        });
    }

    /// Join a voice channel. Returns when the connection is ready.
    ///
    /// The caller must feed `VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE`
    /// from the gateway into `handle_voice_state_update` / `handle_voice_server_update`.
    ///
    /// # Errors
    /// Returns error string if voice connection fails or times out.
    pub async fn join(
        &mut self,
        guild_id: &str,
        channel_id: &str,
        user_id: &str,
    ) -> Result<(), String> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        self.pending.insert(
            channel_id.to_string(),
            PendingVoice {
                guild_id: guild_id.to_string(),
                channel_id: channel_id.to_string(),
                user_id: user_id.to_string(),
                server: None,
                state: None,
                tx,
            },
        );

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(20),
            rx.recv(),
        )
        .await
        .map_err(|_| "voice connection timeout".to_string())?
        .ok_or("voice channel closed")?;

        result
    }

    /// Leave all voice channels in a guild.
    pub fn leave(&mut self, guild_id: &str) {
        let to_remove: Vec<String> = self
            .connections
            .iter()
            .filter(|(_, c)| c.guild_id == guild_id)
            .map(|(k, _)| k.clone())
            .collect();

        for key in to_remove {
            if let Some(mut conn) = self.connections.remove(&key) {
                conn.destroy();
            }
            self.connection_ids.remove(&key);
        }
    }

    /// Leave a specific voice channel.
    pub fn leave_channel(&mut self, channel_id: &str) {
        if let Some(mut conn) = self.connections.remove(channel_id) {
            conn.destroy();
        }
        self.connection_ids.remove(channel_id);
    }

    /// Get an active connection by channel ID.
    pub fn get_connection(&self, channel_id: &str) -> Option<&VoiceConnection> {
        self.connections.get(channel_id)
    }
}
