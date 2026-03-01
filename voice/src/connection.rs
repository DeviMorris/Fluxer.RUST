use std::net::SocketAddr;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message as WsMessage;

const VOICE_VERSION: u8 = 4;
const OPUS_FRAME_TICKS: u32 = 1920;


/// Voice WebSocket opcodes.
pub mod op {
    pub const IDENTIFY: u8 = 0;
    pub const SELECT_PROTOCOL: u8 = 1;
    pub const READY: u8 = 2;
    pub const HEARTBEAT: u8 = 3;
    pub const SESSION_DESCRIPTION: u8 = 4;
    pub const SPEAKING: u8 = 5;
}

/// Events emitted by a voice connection.
#[derive(Debug)]
pub enum VoiceEvent {
    Ready,
    Error(String),
    Disconnect,
}

/// State of the voice connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceState {
    Idle,
    Connecting,
    Connected,
    Playing,
    Disconnected,
}

/// A voice connection using the Fluxer voice gateway protocol.
///
/// Handles voice WebSocket (identify, heartbeat), UDP IP discovery,
/// select protocol, and encrypted audio streaming.
///
/// # Notes
/// - Audio playback (`play_opus`) requires pre-encoded Opus frames.
/// - Encryption uses xsalsa20_poly1305 (requires `crypto_secretbox` or equivalent).
///   For now, the basic frame structure is provided without encryption
///   (encryption can be added via `xsalsa20poly1305` crate).
pub struct VoiceConnection {
    pub guild_id: String,
    pub channel_id: String,
    user_id: String,
    session_id: Option<String>,
    token: Option<String>,
    endpoint: Option<String>,
    ssrc: u32,
    secret_key: Option<Vec<u8>>,
    sequence: u16,
    timestamp: u32,
    playing: bool,
    state: VoiceState,
    tx: mpsc::UnboundedSender<VoiceEvent>,
}

impl VoiceConnection {
    pub fn new(
        guild_id: &str,
        channel_id: &str,
        user_id: &str,
        tx: mpsc::UnboundedSender<VoiceEvent>,
    ) -> Self {
        Self {
            guild_id: guild_id.to_string(),
            channel_id: channel_id.to_string(),
            user_id: user_id.to_string(),
            session_id: None,
            token: None,
            endpoint: None,
            ssrc: 0,
            secret_key: None,
            sequence: 0,
            timestamp: 0,
            playing: false,
            state: VoiceState::Idle,
            tx,
        }
    }

    /// Current state of the connection.
    pub fn state(&self) -> VoiceState {
        self.state
    }

    /// Whether audio is currently playing.
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Connect to the voice server.
    ///
    /// `server_data` must contain `token`, `endpoint`, and optionally `guild_id`.
    /// `state_data` must contain `session_id`.
    ///
    /// # Errors
    /// Returns an error string if connection fails.
    pub async fn connect(
        &mut self,
        server_data: &Value,
        state_data: &Value,
    ) -> Result<(), String> {
        self.state = VoiceState::Connecting;

        let token = server_data
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("missing voice token")?
            .to_string();
        let raw_endpoint = server_data
            .get("endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let session_id = state_data
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if raw_endpoint.is_empty() || token.is_empty() {
            return Err("missing voice server data".to_string());
        }

        self.token = Some(token.clone());
        self.session_id = Some(session_id.clone());

        let host = raw_endpoint
            .replace("wss://", "")
            .replace("ws://", "")
            .split('/')
            .next()
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("")
            .to_string();
        self.endpoint = Some(host.clone());

        let ws_url = if raw_endpoint.contains('?') {
            if raw_endpoint.starts_with("wss://") || raw_endpoint.starts_with("ws://") {
                raw_endpoint.clone()
            } else {
                format!("wss://{raw_endpoint}")
            }
        } else {
            format!("wss://{host}?v={VOICE_VERSION}")
        };

        let (ws_stream, _) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .map_err(|e| format!("voice ws connect failed: {e}"))?;

        let (mut write, mut read) = ws_stream.split();

        let identify = serde_json::json!({
            "op": op::IDENTIFY,
            "d": {
                "server_id": self.guild_id,
                "user_id": self.user_id,
                "session_id": session_id,
                "token": token,
            }
        });
        write
            .send(WsMessage::Text(identify.to_string().into()))
            .await
            .map_err(|e| format!("identify send failed: {e}"))?;

        let mut _udp_socket: Option<UdpSocket> = None;

        while let Some(msg) = read.next().await {
            match msg {
                Ok(WsMessage::Text(text)) => {
                    let payload: Value =
                        serde_json::from_str(&text).unwrap_or(Value::Null);
                    let voice_op = payload.get("op").and_then(|v| v.as_u64()).unwrap_or(255) as u8;
                    let d = payload.get("d").cloned().unwrap_or(Value::Null);

                    match voice_op {
                        op::READY => {
                            self.ssrc = d.get("ssrc").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let port =
                                d.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;
                            let address = d
                                .get("address")
                                .and_then(|v| v.as_str())
                                .unwrap_or(&host);

                            match self.do_udp_discovery(address, port).await {
                                Ok((sock, our_ip, our_port)) => {
                                    let select = serde_json::json!({
                                        "op": op::SELECT_PROTOCOL,
                                        "d": {
                                            "protocol": "udp",
                                            "data": {
                                                "address": our_ip,
                                                "port": our_port,
                                                "mode": "xsalsa20_poly1305"
                                            }
                                        }
                                    });
                                    write
                                        .send(WsMessage::Text(select.to_string().into()))
                                        .await
                                        .map_err(|e| format!("select protocol failed: {e}"))?;
                                    _udp_socket = Some(sock);
                                }
                                Err(e) => {
                                    return Err(format!("UDP discovery failed: {e}"));
                                }
                            }
                        }
                        op::SESSION_DESCRIPTION => {
                            if let Some(key_arr) = d.get("secret_key").and_then(|v| v.as_array()) {
                                let key: Vec<u8> = key_arr
                                    .iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as u8))
                                    .collect();
                                self.secret_key = Some(key);
                            }

                            self.state = VoiceState::Connected;
                            let _ = self.tx.send(VoiceEvent::Ready);

                            let hb_interval = d
                                .get("heartbeat_interval")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(5000);

                            let mut hb_tick = interval(Duration::from_millis(hb_interval));
                            loop {
                                tokio::select! {
                                    _ = hb_tick.tick() => {
                                        let hb = serde_json::json!({
                                            "op": op::HEARTBEAT,
                                            "d": chrono_nanos()
                                        });
                                        let _ = write.send(WsMessage::Text(hb.to_string().into())).await;
                                    }
                                    msg = read.next() => {
                                        match msg {
                                            Some(Ok(WsMessage::Close(_))) | None => {
                                                self.state = VoiceState::Disconnected;
                                                let _ = self.tx.send(VoiceEvent::Disconnect);
                                                return Ok(());
                                            }
                                            Some(Err(e)) => {
                                                let _ = self.tx.send(VoiceEvent::Error(e.to_string()));
                                                self.state = VoiceState::Disconnected;
                                                return Err(e.to_string());
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(WsMessage::Close(_)) => {
                    self.state = VoiceState::Disconnected;
                    let _ = self.tx.send(VoiceEvent::Disconnect);
                    return Ok(());
                }
                Err(e) => {
                    self.state = VoiceState::Disconnected;
                    return Err(format!("voice ws error: {e}"));
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn do_udp_discovery(
        &self,
        address: &str,
        port: u16,
    ) -> Result<(UdpSocket, String, u16), String> {
        let remote: SocketAddr = format!("{address}:{port}")
            .parse()
            .map_err(|e| format!("invalid voice address: {e}"))?;

        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| format!("udp bind: {e}"))?;
        socket
            .connect(remote)
            .await
            .map_err(|e| format!("udp connect: {e}"))?;

        let mut discovery = vec![0u8; 74];
        discovery[0] = 0x00;
        discovery[1] = 0x01;
        discovery[2] = 0x00;
        discovery[3] = 70;
        discovery[4] = (self.ssrc >> 24) as u8;
        discovery[5] = (self.ssrc >> 16) as u8;
        discovery[6] = (self.ssrc >> 8) as u8;
        discovery[7] = self.ssrc as u8;

        socket
            .send(&discovery)
            .await
            .map_err(|e| format!("udp send: {e}"))?;

        let mut buf = vec![0u8; 74];
        let n = tokio::time::timeout(Duration::from_secs(5), socket.recv(&mut buf))
            .await
            .map_err(|_| "UDP discovery timeout".to_string())?
            .map_err(|e| format!("udp recv: {e}"))?;

        if n < 74 {
            return Err("UDP discovery response too short".to_string());
        }

        let ip_end = buf[8..72]
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(64);
        let our_ip =
            String::from_utf8_lossy(&buf[8..8 + ip_end]).to_string();
        let our_port = u16::from_be_bytes([buf[72], buf[73]]);

        Ok((socket, our_ip, our_port))
    }

    /// Build an RTP header for an audio frame.
    ///
    /// Returns 12-byte RTP header. Encryption of the payload
    /// should be done externally (xsalsa20_poly1305).
    pub fn build_rtp_header(&mut self) -> [u8; 12] {
        let mut header = [0u8; 12];
        header[0] = 0x80;
        header[1] = 0x78;
        header[2] = (self.sequence >> 8) as u8;
        header[3] = self.sequence as u8;
        header[4] = (self.timestamp >> 24) as u8;
        header[5] = (self.timestamp >> 16) as u8;
        header[6] = (self.timestamp >> 8) as u8;
        header[7] = self.timestamp as u8;
        header[8] = (self.ssrc >> 24) as u8;
        header[9] = (self.ssrc >> 16) as u8;
        header[10] = (self.ssrc >> 8) as u8;
        header[11] = self.ssrc as u8;
        self.sequence = self.sequence.wrapping_add(1);
        self.timestamp = self.timestamp.wrapping_add(OPUS_FRAME_TICKS);
        header
    }

    /// Stop playback.
    pub fn stop(&mut self) {
        self.playing = false;
    }

    /// Disconnect from voice.
    pub fn disconnect(&mut self) {
        self.playing = false;
        self.state = VoiceState::Disconnected;
        let _ = self.tx.send(VoiceEvent::Disconnect);
    }

    /// Disconnect and clean up.
    pub fn destroy(&mut self) {
        self.disconnect();
        self.secret_key = None;
        self.session_id = None;
        self.token = None;
    }
}

fn chrono_nanos() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
