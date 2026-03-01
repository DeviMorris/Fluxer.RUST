use tokio::sync::mpsc;

use fluxer_types::gateway::{ApiGatewayBotResponse, GatewayPresenceUpdateSendData};

use crate::events::{ShardEvent, WsEvent};
use crate::shard::{ShardOptions, WebSocketShard};

/// Options for the WebSocket manager.
#[derive(Debug, Clone)]
pub struct WebSocketManagerOptions {
    pub token: String,
    pub intents: u64,
    pub presence: Option<GatewayPresenceUpdateSendData>,
    pub shard_ids: Option<Vec<u32>>,
    pub shard_count: Option<u32>,
    pub version: String,
}

impl Default for WebSocketManagerOptions {
    fn default() -> Self {
        Self {
            token: String::new(),
            intents: 0,
            presence: None,
            shard_ids: None,
            shard_count: None,
            version: "1".to_string(),
        }
    }
}

/// Manages gateway WebSocket shards.
///
/// Connects to `GET /gateway/bot`, spawns shards, publishes events
/// through a `tokio::sync::mpsc` channel.
///
/// # Examples
/// ```rust,ignore
/// use fluxer_ws::{WebSocketManager, WebSocketManagerOptions, WsEvent};
///
/// let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
/// let mut manager = WebSocketManager::new(options, rest, tx);
/// manager.connect().await?;
/// while let Some(event) = rx.recv().await {
///     match event {
///         WsEvent::Dispatch { payload, .. } => { /* handle */ }
///         _ => {}
///     }
/// }
/// ```
pub struct WebSocketManager {
    options: WebSocketManagerOptions,
    rest: fluxer_rest::Rest,
    tx: mpsc::UnboundedSender<WsEvent>,
    shard_count: u32,
    gateway_url: Option<String>,
}

impl WebSocketManager {
    pub fn new(
        options: WebSocketManagerOptions,
        rest: fluxer_rest::Rest,
        tx: mpsc::UnboundedSender<WsEvent>,
    ) -> Self {
        Self {
            options,
            rest,
            tx,
            shard_count: 1,
            gateway_url: None,
        }
    }

    /// Connect to the gateway and spawn all shards.
    ///
    /// Fetches `GET /gateway/bot` for the gateway URL and recommended shard count,
    /// then spawns a tokio task per shard.
    ///
    /// # Errors
    /// Returns [`fluxer_rest::RestError`] if the gateway fetch fails.
    pub async fn connect(&mut self) -> Result<(), fluxer_rest::RestError> {
        let gateway: ApiGatewayBotResponse = self.rest.get("/gateway/bot").await?;

        self.gateway_url = Some(gateway.url.clone());
        self.shard_count = self.options.shard_count.unwrap_or(gateway.shards);

        let ids: Vec<u32> = self
            .options
            .shard_ids
            .clone()
            .unwrap_or_else(|| (0..self.shard_count).collect());

        for &shard_id in &ids {
            let shard_opts = ShardOptions {
                url: gateway.url.clone(),
                token: self.options.token.clone(),
                intents: self.options.intents,
                presence: self.options.presence.clone(),
                shard_id,
                num_shards: self.shard_count,
                version: self.options.version.clone(),
            };

            let ws_tx = self.tx.clone();
            let (shard_tx, mut shard_rx) = mpsc::unbounded_channel::<ShardEvent>();

            tokio::spawn(async move {
                let mut shard = WebSocketShard::new(shard_opts, shard_tx);
                shard.run().await;
            });

            let id = shard_id;
            tokio::spawn(async move {
                while let Some(event) = shard_rx.recv().await {
                    let ws_event = match event {
                        ShardEvent::Ready(data) => WsEvent::ShardReady {
                            shard_id: id,
                            data,
                        },
                        ShardEvent::Resumed => WsEvent::ShardResumed { shard_id: id },
                        ShardEvent::Dispatch(payload) => WsEvent::Dispatch {
                            shard_id: id,
                            payload,
                        },
                        ShardEvent::Close(code) => WsEvent::ShardClose {
                            shard_id: id,
                            code,
                        },
                        ShardEvent::Error(msg) => WsEvent::Error {
                            shard_id: id,
                            error: msg,
                        },
                        ShardEvent::Debug(msg) => WsEvent::Debug(msg),
                    };
                    if ws_tx.send(ws_event).is_err() {
                        break;
                    }
                }
            });
        }

        Ok(())
    }

    /// Get the number of shards.
    pub fn shard_count(&self) -> u32 {
        self.shard_count
    }

    /// Get the gateway URL (available after connect).
    pub fn gateway_url(&self) -> Option<&str> {
        self.gateway_url.as_deref()
    }
}
