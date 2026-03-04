use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoiceError {
    #[error("Connection timeout")]
    Timeout,
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Playback error: {0}")]
    PlaybackError(String),
    #[error("LiveKit error: {0}")]
    LiveKit(String),
    #[error("Not connected")]
    NotConnected,
    #[error("Gateway sender unavailable")]
    GatewayUnavailable,
}

impl From<livekit::RoomError> for VoiceError {
    fn from(err: livekit::RoomError) -> Self {
        VoiceError::LiveKit(err.to_string())
    }
}

impl From<livekit::track::TrackError> for VoiceError {
    fn from(err: livekit::track::TrackError) -> Self {
        VoiceError::LiveKit(err.to_string())
    }
}
