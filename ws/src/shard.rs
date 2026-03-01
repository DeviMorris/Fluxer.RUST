use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::{interval, sleep};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tracing::warn;

use fluxer_types::gateway::{
    GatewayHelloData, GatewayIdentifyData, GatewayIdentifyProperties, GatewayOpcode,
    GatewayPresenceUpdateSendData, GatewayReceivePayload, GatewayResumeData,
};

use crate::events::ShardEvent;

const RECONNECT_INITIAL_MS: u64 = 1_000;
const RECONNECT_MAX_MS: u64 = 45_000;

/// Configuration for a single WebSocket shard.
#[derive(Debug, Clone)]
pub struct ShardOptions {
    pub url: String,
    pub token: String,
    pub intents: u64,
    pub presence: Option<GatewayPresenceUpdateSendData>,
    pub shard_id: u32,
    pub num_shards: u32,
    pub version: String,
}

/// A single WebSocket shard connecting to the Fluxer gateway.
///
/// Handles heartbeating, identify/resume, and auto-reconnect with exponential backoff.
/// Emits events through an `mpsc` channel.
pub struct WebSocketShard {
    options: ShardOptions,
    session_id: Option<String>,
    seq: Option<u64>,
    destroying: bool,
    reconnect_delay_ms: u64,
    tx: mpsc::UnboundedSender<ShardEvent>,
}

impl WebSocketShard {
    pub fn new(options: ShardOptions, tx: mpsc::UnboundedSender<ShardEvent>) -> Self {
        Self {
            options,
            session_id: None,
            seq: None,
            destroying: false,
            reconnect_delay_ms: RECONNECT_INITIAL_MS,
            tx,
        }
    }

    /// Connect to the gateway and run the event loop.
    ///
    /// This method spawns the heartbeat task and processes incoming messages.
    /// On disconnect it will auto-reconnect unless `destroy()` has been called.
    pub async fn run(&mut self) {
        loop {
            if self.destroying {
                return;
            }

            let url = format!(
                "{}/?v={}&encoding=json",
                self.options.url, self.options.version
            );

            self.emit(ShardEvent::Debug(format!(
                "[Shard {}] Connecting to {url}",
                self.options.shard_id
            )));

            let ws_stream = match tokio_tungstenite::connect_async_tls_with_config(
                &url,
                None,
                false,
                Some(tokio_tungstenite::Connector::NativeTls(
                    native_tls::TlsConnector::new().unwrap(),
                )),
            )
            .await
            {
                Ok((stream, _)) => stream,
                Err(e) => {
                    self.emit(ShardEvent::Error(format!("Connect error: {e}")));
                    if self.destroying {
                        return;
                    }
                    self.schedule_reconnect().await;
                    continue;
                }
            };

            self.reconnect_delay_ms = RECONNECT_INITIAL_MS;
            let (mut write, mut read) = ws_stream.split();

            let (hb_tx, mut hb_rx) = mpsc::unbounded_channel::<Value>();
            let mut heartbeat_interval_ms = None;
            let mut last_heartbeat_ack = true;

            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(WsMessage::Text(text))) => {
                                match serde_json::from_str::<GatewayReceivePayload>(&text) {
                                    Ok(payload) => {
                                        match payload.op {
                                            GatewayOpcode::Hello => {
                                                if let Some(d) = &payload.d {
                                                    if let Ok(hello) = serde_json::from_value::<GatewayHelloData>(d.clone()) {
                                                        heartbeat_interval_ms = Some(hello.heartbeat_interval);
                                                        last_heartbeat_ack = true;

                                                        let hb_ms = hello.heartbeat_interval;
                                                        let hb_tx_clone = hb_tx.clone();
                                                        let seq = self.seq;
                                                        tokio::spawn(async move {
                                                            run_heartbeat(hb_ms, hb_tx_clone, seq).await;
                                                        });

                                                        let identify_payload = self.build_identify_or_resume();
                                                        let json = serde_json::to_string(&identify_payload)
                                                            .unwrap_or_default();
                                                        let _ = write.send(WsMessage::Text(json.into())).await;
                                                    }
                                                }
                                            }
                                            GatewayOpcode::HeartbeatAck => {
                                                last_heartbeat_ack = true;
                                            }
                                            GatewayOpcode::Dispatch => {
                                                if let Some(s) = payload.s {
                                                    self.seq = Some(s);
                                                }
                                                if payload.t.as_deref() == Some("READY") {
                                                    if let Some(d) = &payload.d {
                                                        if let Some(sid) = d.get("session_id").and_then(|v| v.as_str()) {
                                                            self.session_id = Some(sid.to_string());
                                                        }
                                                    }
                                                    self.reconnect_delay_ms = RECONNECT_INITIAL_MS;
                                                    self.emit(ShardEvent::Ready(
                                                        payload.d.clone().unwrap_or(Value::Null),
                                                    ));
                                                } else if payload.t.as_deref() == Some("RESUMED") {
                                                    self.reconnect_delay_ms = RECONNECT_INITIAL_MS;
                                                    self.emit(ShardEvent::Resumed);
                                                }
                                                self.emit(ShardEvent::Dispatch(payload));
                                            }
                                            GatewayOpcode::InvalidSession => {
                                                self.emit(ShardEvent::Debug(format!(
                                                    "[Shard {}] Invalid session, reconnecting",
                                                    self.options.shard_id
                                                )));
                                                self.session_id = None;
                                                self.seq = None;
                                                sleep(Duration::from_millis(1000 + rand_u64(4000))).await;
                                                break;
                                            }
                                            GatewayOpcode::Reconnect => {
                                                self.emit(ShardEvent::Debug(format!(
                                                    "[Shard {}] Reconnect requested",
                                                    self.options.shard_id
                                                )));
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse gateway payload: {e}");
                                    }
                                }
                            }
                            Some(Ok(WsMessage::Close(frame))) => {
                                let code = frame.as_ref().map(|f| f.code.into()).unwrap_or(1006u16);
                                self.emit(ShardEvent::Close(code));
                                self.emit(ShardEvent::Debug(format!(
                                    "[Shard {}] Closed: {code}",
                                    self.options.shard_id
                                )));
                                if !self.destroying && should_reconnect_on_close(code) {
                                    break;
                                }
                                return;
                            }
                            Some(Err(e)) => {
                                self.emit(ShardEvent::Error(format!("WS error: {e}")));
                                break;
                            }
                            None => {
                                self.emit(ShardEvent::Close(1006));
                                break;
                            }
                            _ => {}
                        }
                    }
                    hb = hb_rx.recv() => {
                        if let Some(payload) = hb {
                            if !last_heartbeat_ack && self.seq.is_some() {
                                self.emit(ShardEvent::Debug(format!(
                                    "[Shard {}] Heartbeat ack missed; reconnecting",
                                    self.options.shard_id
                                )));
                                break;
                            }
                            last_heartbeat_ack = false;
                            let json = serde_json::to_string(&payload).unwrap_or_default();
                            let _ = write.send(WsMessage::Text(json.into())).await;
                        }
                    }
                }
            }

            let _ = heartbeat_interval_ms;
            if self.destroying {
                return;
            }
            self.schedule_reconnect().await;
        }
    }

    fn build_identify_or_resume(&self) -> Value {
        if let (Some(session_id), Some(seq)) = (&self.session_id, self.seq) {
            let resume = GatewayResumeData {
                token: self.options.token.clone(),
                session_id: session_id.clone(),
                seq,
            };
            serde_json::json!({
                "op": GatewayOpcode::Resume as u8,
                "d": resume
            })
        } else {
            let identify = GatewayIdentifyData {
                token: self.options.token.clone(),
                intents: self.options.intents,
                properties: GatewayIdentifyProperties {
                    os: std::env::consts::OS.to_string(),
                    browser: "fluxer-rust".to_string(),
                    device: "fluxer-rust".to_string(),
                },
                compress: None,
                large_threshold: None,
                shard: Some((self.options.shard_id, self.options.num_shards)),
                presence: self.options.presence.clone(),
            };
            serde_json::json!({
                "op": GatewayOpcode::Identify as u8,
                "d": identify
            })
        }
    }

    async fn schedule_reconnect(&mut self) {
        let jitter = (self.reconnect_delay_ms as f64) * (0.75 + rand_f64() * 0.5);
        let delay = jitter.min(RECONNECT_MAX_MS as f64) as u64;
        self.reconnect_delay_ms =
            (self.reconnect_delay_ms as f64 * 1.5).min(RECONNECT_MAX_MS as f64) as u64;
        self.emit(ShardEvent::Debug(format!(
            "[Shard {}] Reconnecting in {delay}ms…",
            self.options.shard_id
        )));
        sleep(Duration::from_millis(delay)).await;
    }

    fn emit(&self, event: ShardEvent) {
        let _ = self.tx.send(event);
    }

    /// Signal the shard to stop and not reconnect.
    pub fn destroy(&mut self) {
        self.destroying = true;
        self.session_id = None;
        self.seq = None;
    }
}

async fn run_heartbeat(interval_ms: u64, tx: mpsc::UnboundedSender<Value>, initial_seq: Option<u64>) {
    let jitter = Duration::from_millis((interval_ms as f64 * rand_f64()) as u64);
    sleep(jitter).await;

    let mut tick = interval(Duration::from_millis(interval_ms));
    loop {
        tick.tick().await;
        let payload = serde_json::json!({
            "op": GatewayOpcode::Heartbeat as u8,
            "d": initial_seq
        });
        if tx.send(payload).is_err() {
            break;
        }
    }
}

fn should_reconnect_on_close(code: u16) -> bool {
    matches!(
        code,
        1000 | 1001 | 1005 | 1006 | 1011 | 1012 | 1013 | 1014 | 1015
            | 4000 | 4007 | 4009 | 4010 | 4011 | 4012
    )
}

fn rand_f64() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos as f64) / 1_000_000_000.0
}

fn rand_u64(max: u64) -> u64 {
    (rand_f64() * max as f64) as u64
}
