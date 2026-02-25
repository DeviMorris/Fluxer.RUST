use crate::error::{Result, StateError, TransportError};
use crate::gateway::dispatch::{DispatchEnvelope, decode_dispatch};
use crate::gateway::rate_limiter::{OutboundKind, OutboundRateLimiter};
use crate::gateway::transport::GatewayTransport;
use futures_util::future::pending;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, Notify, RwLock, broadcast, mpsc};
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant, sleep};
use tokio_tungstenite::connect_async;
use url::Url;

const GATEWAY_VERSION: u8 = 1;
const MAX_RECONNECT_DELAY: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionMode {
    None,
    ZlibPayload,
    ZlibStream,
    ZstdStream,
}

impl CompressionMode {
    fn query_value(self) -> Option<&'static str> {
        match self {
            Self::None | Self::ZlibPayload => None,
            Self::ZlibStream => Some("zlib-stream"),
            Self::ZstdStream => Some("zstd-stream"),
        }
    }

    fn identify_compress(self) -> bool {
        matches!(self, Self::ZlibPayload)
    }
}

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub url: String,
    pub token: String,
    pub compression: CompressionMode,
    pub commands_per_minute: u32,
    pub reserved_slots: u32,
    pub reconnect_backoff_base: Duration,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            url: "wss://gateway.fluxer.app".to_owned(),
            token: String::new(),
            compression: CompressionMode::None,
            commands_per_minute: 120,
            reserved_slots: 3,
            reconnect_backoff_base: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayStatus {
    Unconnected,
    Connecting,
    WaitingHello,
    Identifying,
    Resuming,
    Ready,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeState {
    pub session_id: String,
    pub seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayEvent {
    pub op: u8,
    #[serde(default)]
    pub s: Option<u64>,
    #[serde(default)]
    pub t: Option<String>,
    #[serde(default)]
    pub d: Value,
}

#[derive(Debug, Clone)]
/// Gateway transport client for Fluxer websocket sessions.
///
/// Manages websocket connect/reconnect, heartbeat/resume and outbound command flow.
pub struct GatewayClient {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    cfg: GatewayConfig,
    status: RwLock<GatewayStatus>,
    resume: RwLock<Option<ResumeState>>,
    outbound_tx: Mutex<Option<mpsc::Sender<OutboundCommand>>>,
    events_tx: broadcast::Sender<GatewayEvent>,
    dispatch_tx: broadcast::Sender<DispatchEnvelope>,
    shutdown: Notify,
    closed: AtomicBool,
    runner: Mutex<Option<JoinHandle<()>>>,
}

#[derive(Debug)]
struct OutboundCommand {
    kind: OutboundKind,
    op: u8,
    d: Value,
}

enum LoopAction {
    Reconnect,
    Shutdown,
}

impl GatewayClient {
    pub fn new(cfg: GatewayConfig) -> Self {
        let (events_tx, _) = broadcast::channel(256);
        let (dispatch_tx, _) = broadcast::channel(256);
        Self {
            inner: Arc::new(Inner {
                cfg,
                status: RwLock::new(GatewayStatus::Unconnected),
                resume: RwLock::new(None),
                outbound_tx: Mutex::new(None),
                events_tx,
                dispatch_tx,
                shutdown: Notify::new(),
                closed: AtomicBool::new(true),
                runner: Mutex::new(None),
            }),
        }
    }

    pub async fn open(&self) -> Result<()> {
        let mut runner = self.inner.runner.lock().await;
        if runner.is_some() {
            return Ok(());
        }

        let (tx, rx) = mpsc::channel(256);
        *self.inner.outbound_tx.lock().await = Some(tx);
        self.inner.closed.store(false, Ordering::SeqCst);

        let inner = self.inner.clone();
        *runner = Some(tokio::spawn(async move {
            run(inner, rx).await;
        }));

        Ok(())
    }

    pub async fn close(&self) {
        self.inner.closed.store(true, Ordering::SeqCst);
        self.inner.shutdown.notify_waiters();
        *self.inner.outbound_tx.lock().await = None;

        if let Some(handle) = self.inner.runner.lock().await.take() {
            let _ = handle.await;
        }
    }

    pub async fn send(&self, op: u8, data: Value) -> Result<()> {
        let tx = self
            .inner
            .outbound_tx
            .lock()
            .await
            .clone()
            .ok_or(StateError::NotConnected)?;
        tx.send(OutboundCommand {
            kind: OutboundKind::Normal,
            op,
            d: data,
        })
        .await
        .map_err(|_| StateError::Closed)?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<GatewayEvent> {
        self.inner.events_tx.subscribe()
    }

    pub fn subscribe_dispatch(&self) -> broadcast::Receiver<DispatchEnvelope> {
        self.inner.dispatch_tx.subscribe()
    }

    pub async fn status(&self) -> GatewayStatus {
        *self.inner.status.read().await
    }

    pub async fn resume_state(&self) -> Option<ResumeState> {
        self.inner.resume.read().await.clone()
    }
}

async fn run(inner: Arc<Inner>, mut outbound_rx: mpsc::Receiver<OutboundCommand>) {
    let mut attempts = 0u32;
    loop {
        if inner.closed.load(Ordering::SeqCst) {
            set_status(&inner, GatewayStatus::Disconnected).await;
            return;
        }

        set_status(&inner, GatewayStatus::Connecting).await;
        match connect_transport(&inner.cfg).await {
            Ok(mut transport) => {
                attempts = 0;
                let mut limiter = OutboundRateLimiter::new(
                    inner.cfg.commands_per_minute,
                    inner.cfg.reserved_slots,
                );
                limiter.reset();
                set_status(&inner, GatewayStatus::WaitingHello).await;

                match run_connection(&inner, &mut transport, &mut limiter, &mut outbound_rx).await {
                    Ok(LoopAction::Shutdown) => {
                        let _ = transport.close().await;
                        set_status(&inner, GatewayStatus::Disconnected).await;
                        return;
                    }
                    Ok(LoopAction::Reconnect) | Err(_) => {
                        let _ = transport.close().await;
                        set_status(&inner, GatewayStatus::Disconnected).await;
                    }
                }
            }
            Err(_) => {
                set_status(&inner, GatewayStatus::Disconnected).await;
            }
        }

        attempts = attempts.saturating_add(1);
        let delay = reconnect_delay(inner.cfg.reconnect_backoff_base, attempts);
        tokio::select! {
            _ = sleep(delay) => {}
            _ = inner.shutdown.notified() => return,
        }
    }
}

async fn run_connection(
    inner: &Arc<Inner>,
    transport: &mut GatewayTransport,
    limiter: &mut OutboundRateLimiter,
    outbound_rx: &mut mpsc::Receiver<OutboundCommand>,
) -> Result<LoopAction> {
    let mut heartbeat_interval = None;
    let mut heartbeat_due: Option<Instant> = None;
    let mut awaiting_ack = false;

    loop {
        tokio::select! {
            _ = inner.shutdown.notified() => {
                return Ok(LoopAction::Shutdown);
            }
            _ = heartbeat_tick(heartbeat_due) => {
                if awaiting_ack {
                    return Ok(LoopAction::Reconnect);
                }
                let seq = inner.resume.read().await.as_ref().map(|r| r.seq);
                send_command(
                    transport,
                    limiter,
                    &inner.shutdown,
                    OutboundKind::Internal,
                    1,
                    seq.map_or(Value::Null, Value::from),
                ).await?;
                awaiting_ack = true;
                if let Some(interval) = heartbeat_interval {
                    heartbeat_due = Some(Instant::now() + interval);
                }
            }
            cmd = outbound_rx.recv() => {
                let Some(cmd) = cmd else {
                    return Ok(LoopAction::Shutdown);
                };
                send_command(
                    transport,
                    limiter,
                    &inner.shutdown,
                    cmd.kind,
                    cmd.op,
                    cmd.d,
                ).await?;
            }
            msg = transport.recv() => {
                let Some(event) = msg? else {
                    return Ok(LoopAction::Reconnect);
                };

                if let Some(seq) = event.s {
                    update_seq(inner, seq).await;
                }

                match event.op {
                    10 => {
                        let interval = parse_heartbeat_interval(&event).ok_or_else(|| {
                            TransportError::Other("HELLO without heartbeat_interval".to_owned())
                        })?;
                        heartbeat_interval = Some(interval);
                        heartbeat_due = Some(Instant::now() + interval);

                        let resume = inner.resume.read().await.clone();
                        if let Some(resume) = resume {
                            set_status(inner, GatewayStatus::Resuming).await;
                            send_command(
                                transport,
                                limiter,
                                &inner.shutdown,
                                OutboundKind::Internal,
                                6,
                                serde_json::json!({
                                    "token": inner.cfg.token,
                                    "session_id": resume.session_id,
                                    "seq": resume.seq
                                }),
                            ).await?;
                        } else {
                            set_status(inner, GatewayStatus::Identifying).await;
                            send_command(
                                transport,
                                limiter,
                                &inner.shutdown,
                                OutboundKind::Internal,
                                2,
                                serde_json::json!({
                                    "token": inner.cfg.token,
                                    "properties": {
                                        "os": std::env::consts::OS,
                                        "browser": "fluxer-rust",
                                        "device": "fluxer-rust"
                                    },
                                    "compress": inner.cfg.compression.identify_compress()
                                }),
                            ).await?;
                        }
                    }
                    1 => {
                        let seq = inner.resume.read().await.as_ref().map(|r| r.seq);
                        send_command(
                            transport,
                            limiter,
                            &inner.shutdown,
                            OutboundKind::Internal,
                            1,
                            seq.map_or(Value::Null, Value::from),
                        ).await?;
                    }
                    0 => {
                        if let Some(seq) = event.s {
                            update_seq(inner, seq).await;
                        }

                        if event.t.as_deref() == Some("READY") {
                            if let Some(session_id) = event.d.get("session_id").and_then(Value::as_str) {
                                let seq = event.s.unwrap_or(0);
                                *inner.resume.write().await = Some(ResumeState {
                                    session_id: session_id.to_owned(),
                                    seq,
                                });
                            }
                            set_status(inner, GatewayStatus::Ready).await;
                        } else if event.t.as_deref() == Some("RESUMED") {
                            set_status(inner, GatewayStatus::Ready).await;
                        }
                    }
                    7 => return Ok(LoopAction::Reconnect),
                    9 => {
                        let can_resume = event.d.as_bool().unwrap_or(false);
                        if !can_resume {
                            *inner.resume.write().await = None;
                        }
                        return Ok(LoopAction::Reconnect);
                    }
                    11 => {
                        awaiting_ack = false;
                    }
                    _ => {}
                }

                if let Some(dispatch) = decode_dispatch(&event)? {
                    let _ = inner.dispatch_tx.send(dispatch);
                }
                let _ = inner.events_tx.send(event);
            }
        }
    }
}

async fn connect_transport(cfg: &GatewayConfig) -> Result<GatewayTransport> {
    let url = build_gateway_url(cfg)?;
    let (ws, _) = connect_async(url.as_str())
        .await
        .map_err(|e| TransportError::Other(e.to_string()))?;
    Ok(GatewayTransport::new(ws, cfg.compression))
}

fn build_gateway_url(cfg: &GatewayConfig) -> Result<Url> {
    let mut url = Url::parse(&cfg.url)
        .map_err(|e| TransportError::Other(format!("invalid gateway url: {e}")))?;
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("v", &GATEWAY_VERSION.to_string());
        q.append_pair("encoding", "json");
        if let Some(compress) = cfg.compression.query_value() {
            q.append_pair("compress", compress);
        }
    }
    Ok(url)
}

async fn send_command(
    transport: &mut GatewayTransport,
    limiter: &mut OutboundRateLimiter,
    shutdown: &Notify,
    kind: OutboundKind,
    op: u8,
    d: Value,
) -> Result<()> {
    limiter.wait(kind, shutdown).await?;
    transport
        .send_json(&serde_json::json!({ "op": op, "d": d }))
        .await
}

async fn update_seq(inner: &Arc<Inner>, seq: u64) {
    let mut guard = inner.resume.write().await;
    if let Some(mut resume) = guard.clone() {
        resume.seq = seq;
        *guard = Some(resume);
    }
}

fn reconnect_delay(base: Duration, attempts: u32) -> Duration {
    let exp = 1u128 << attempts.min(10);
    let millis = base.as_millis().saturating_mul(exp);
    Duration::from_millis(millis.min(MAX_RECONNECT_DELAY.as_millis()) as u64)
}

fn parse_heartbeat_interval(event: &GatewayEvent) -> Option<Duration> {
    event
        .d
        .get("heartbeat_interval")
        .and_then(Value::as_u64)
        .map(Duration::from_millis)
}

async fn set_status(inner: &Arc<Inner>, status: GatewayStatus) {
    *inner.status.write().await = status;
}

async fn heartbeat_tick(next: Option<Instant>) {
    match next {
        Some(instant) => {
            let now = Instant::now();
            if instant > now {
                sleep(instant - now).await;
            }
        }
        None => pending::<()>().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_caps() {
        let d = reconnect_delay(Duration::from_secs(2), 30);
        assert_eq!(d, MAX_RECONNECT_DELAY);
    }

    #[test]
    fn heartbeat_parse() {
        let ev = GatewayEvent {
            op: 10,
            s: None,
            t: None,
            d: serde_json::json!({ "heartbeat_interval": 45000 }),
        };
        assert_eq!(
            parse_heartbeat_interval(&ev),
            Some(Duration::from_millis(45000))
        );
    }

    #[test]
    fn url_build() {
        let cfg = GatewayConfig {
            compression: CompressionMode::ZlibStream,
            ..GatewayConfig::default()
        };
        let url = build_gateway_url(&cfg).expect("url");
        let query = url.query().expect("query");
        assert!(query.contains("compress=zlib-stream"));
    }
}
