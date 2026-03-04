use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::{RwLock, mpsc};

use fluxer_types::gateway::{ApiGatewayBotResponse, GatewayPresenceUpdateSendData};

use crate::events::{ShardEvent, WsEvent};
use crate::shard::{ShardOptions, WebSocketShard};

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

pub struct WebSocketManager {
    options: WebSocketManagerOptions,
    rest: fluxer_rest::Rest,
    tx: mpsc::UnboundedSender<WsEvent>,
    shard_count: u32,
    gateway_url: Option<String>,
    shard_senders: Arc<RwLock<HashMap<u32, mpsc::UnboundedSender<Value>>>>,
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
            shard_senders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

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
            let (user_tx, user_rx) = mpsc::unbounded_channel::<Value>();

            {
                let mut senders = self.shard_senders.write().await;
                senders.insert(shard_id, user_tx);
            }

            tokio::spawn(async move {
                let mut shard = WebSocketShard::new(shard_opts, shard_tx, user_rx);
                shard.run().await;
            });

            let id = shard_id;
            tokio::spawn(async move {
                while let Some(event) = shard_rx.recv().await {
                    let ws_event = match event {
                        ShardEvent::Ready(data) => WsEvent::ShardReady { shard_id: id, data },
                        ShardEvent::Resumed => WsEvent::ShardResumed { shard_id: id },
                        ShardEvent::Dispatch(payload) => WsEvent::Dispatch {
                            shard_id: id,
                            payload,
                        },
                        ShardEvent::Close(code) => WsEvent::ShardClose { shard_id: id, code },
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

    pub async fn send(&self, shard_id: u32, payload: Value) -> Result<(), String> {
        let senders = self.shard_senders.read().await;
        match senders.get(&shard_id) {
            Some(tx) => tx
                .send(payload)
                .map_err(|_| format!("Shard {shard_id} channel closed")),
            None => Err(format!("Shard {shard_id} not found")),
        }
    }

    pub async fn broadcast(&self, payload: Value) {
        let senders = self.shard_senders.read().await;
        for (_, tx) in senders.iter() {
            let _ = tx.send(payload.clone());
        }
    }

    pub fn shard_count(&self) -> u32 {
        self.shard_count
    }

    pub fn gateway_url(&self) -> Option<&str> {
        self.gateway_url.as_deref()
    }

    pub fn shard_senders(&self) -> Arc<RwLock<HashMap<u32, mpsc::UnboundedSender<Value>>>> {
        self.shard_senders.clone()
    }
}
