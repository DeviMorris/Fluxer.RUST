use crate::error::{ProtocolError, Result, TransportError};
use crate::gateway::client::{CompressionMode, GatewayEvent};
use flate2::Decompress;
use flate2::FlushDecompress;
use futures_util::{SinkExt, StreamExt};
use std::io::Read;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type Ws = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct GatewayTransport {
    ws: Ws,
    decoder: PayloadDecoder,
}

impl GatewayTransport {
    pub fn new(ws: Ws, mode: CompressionMode) -> Self {
        Self {
            ws,
            decoder: PayloadDecoder::new(mode),
        }
    }

    pub async fn recv(&mut self) -> Result<Option<GatewayEvent>> {
        while let Some(frame) = self.ws.next().await {
            let msg = frame.map_err(|e| TransportError::Other(e.to_string()))?;
            match msg {
                Message::Text(text) => return Ok(Some(parse_event(text.as_bytes())?)),
                Message::Binary(data) => {
                    if let Some(payload) = self.decoder.decode_binary(&data)? {
                        return Ok(Some(parse_event(&payload)?));
                    }
                }
                Message::Close(_) => return Ok(None),
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Frame(_) => {}
            }
        }
        Ok(None)
    }

    pub async fn send_json(&mut self, value: &serde_json::Value) -> Result<()> {
        let text = serde_json::to_string(value)
            .map_err(|e| ProtocolError::InvalidPayload(e.to_string()))?;
        self.ws
            .send(Message::Text(text))
            .await
            .map_err(|e| TransportError::Other(e.to_string()).into())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.ws
            .close(None)
            .await
            .map_err(|e| TransportError::Other(e.to_string()).into())
    }
}

struct PayloadDecoder {
    mode: CompressionMode,
    zlib_stream: Vec<u8>,
    zlib_inflater: Decompress,
    zstd_stream: Vec<u8>,
}

impl PayloadDecoder {
    fn new(mode: CompressionMode) -> Self {
        Self {
            mode,
            zlib_stream: Vec::new(),
            zlib_inflater: Decompress::new(true),
            zstd_stream: Vec::new(),
        }
    }

    fn decode_binary(&mut self, data: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.mode {
            CompressionMode::None => Ok(Some(data.to_vec())),
            CompressionMode::ZlibPayload => decode_zlib_payload(data).map(Some),
            CompressionMode::ZlibStream => self.decode_zlib_stream(data),
            CompressionMode::ZstdStream => self.decode_zstd_stream(data),
        }
    }

    fn decode_zlib_stream(&mut self, data: &[u8]) -> Result<Option<Vec<u8>>> {
        self.zlib_stream.extend_from_slice(data);
        if !ends_with_sync_flush(&self.zlib_stream) {
            return Ok(None);
        }

        let before = self.zlib_inflater.total_in();
        let mut out = Vec::new();
        self.zlib_inflater
            .decompress_vec(&self.zlib_stream, &mut out, FlushDecompress::Sync)
            .map_err(|e| TransportError::Other(format!("zlib stream decode error: {e}")))?;
        let consumed = (self.zlib_inflater.total_in() - before) as usize;
        if consumed > 0 && consumed <= self.zlib_stream.len() {
            self.zlib_stream.drain(0..consumed);
        } else {
            self.zlib_stream.clear();
        }

        Ok(Some(out))
    }

    fn decode_zstd_stream(&mut self, data: &[u8]) -> Result<Option<Vec<u8>>> {
        self.zstd_stream.extend_from_slice(data);
        match zstd::stream::decode_all(&self.zstd_stream[..]) {
            Ok(decoded) => {
                self.zstd_stream.clear();
                Ok(Some(decoded))
            }
            Err(_) => Ok(None),
        }
    }
}

fn parse_event(payload: &[u8]) -> Result<GatewayEvent> {
    serde_json::from_slice(payload)
        .map_err(|e| ProtocolError::InvalidPayload(format!("gateway payload: {e}")).into())
}

fn decode_zlib_payload(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| TransportError::Other(format!("zlib payload decode error: {e}")))?;
    Ok(out)
}

fn ends_with_sync_flush(data: &[u8]) -> bool {
    data.len() >= 4 && data[data.len() - 4..] == [0x00, 0x00, 0xff, 0xff]
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    #[test]
    fn zlib_decode() {
        let input = br#"{"op":1}"#;
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
        enc.write_all(input).expect("write");
        let compressed = enc.finish().expect("finish");
        let out = decode_zlib_payload(&compressed).expect("decode");
        assert_eq!(out, input);
    }

    #[test]
    fn sync_flush() {
        assert!(ends_with_sync_flush(&[1, 2, 0, 0, 255, 255]));
        assert!(!ends_with_sync_flush(&[1, 2, 3]));
    }
}
