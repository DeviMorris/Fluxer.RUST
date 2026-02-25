use core::fmt;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    Transport,
    Protocol,
    Api,
    RateLimit,
    State,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    RateLimit(#[from] RateLimitError),
    #[error(transparent)]
    State(#[from] StateError),
}

impl Error {
    pub const fn category(&self) -> ErrorCategory {
        match self {
            Self::Transport(_) => ErrorCategory::Transport,
            Self::Protocol(_) => ErrorCategory::Protocol,
            Self::Api(_) => ErrorCategory::Api,
            Self::RateLimit(_) => ErrorCategory::RateLimit,
            Self::State(_) => ErrorCategory::State,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Transport(TransportError::Io(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Protocol(ProtocolError::Json(value))
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("i/o error: {0}")]
    Io(#[source] std::io::Error),
    #[error("request timed out")]
    Timeout,
    #[error("request canceled")]
    Canceled,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("json decode/encode error: {0}")]
    Json(#[source] serde_json::Error),
    #[error("invalid payload: {0}")]
    InvalidPayload(String),
    #[error("invalid route template: {0}")]
    InvalidRouteTemplate(String),
    #[error("missing route parameter: {0}")]
    MissingRouteParam(String),
    #[error("unexpected opcode: expected {expected}, got {got}")]
    UnexpectedOpcode { expected: u8, got: u8 },
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub status: u16,
    pub code: Option<i64>,
    pub message: String,
}

impl ApiError {
    pub fn new(status: u16, code: Option<i64>, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                f,
                "api error (status {}, code {}): {}",
                self.status, code, self.message
            ),
            None => write!(f, "api error (status {}): {}", self.status, self.message),
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitError {
    pub retry_after: Duration,
    pub bucket: Option<String>,
    pub global: bool,
}

impl RateLimitError {
    pub fn new(retry_after: Duration, bucket: Option<String>, global: bool) -> Self {
        Self {
            retry_after,
            bucket,
            global,
        }
    }
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rate limited for {:?}", self.retry_after)?;
        if let Some(bucket) = self.bucket.as_deref() {
            write!(f, ", bucket {}", bucket)?;
        }
        if self.global {
            write!(f, ", global")
        } else {
            write!(f, ", route")
        }
    }
}

impl std::error::Error for RateLimitError {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StateError {
    #[error("client is not connected")]
    NotConnected,
    #[error("client is already running")]
    AlreadyRunning,
    #[error("client is closed")]
    Closed,
    #[error("missing required state: {0}")]
    Missing(&'static str),
    #[error("invalid state transition: {from} -> {to}")]
    InvalidTransition {
        from: &'static str,
        to: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category() {
        let err = Error::from(TransportError::Timeout);
        assert_eq!(err.category(), ErrorCategory::Transport);
    }

    #[test]
    fn api_msg_with_code() {
        let err = ApiError::new(400, Some(1001), "bad request");
        assert_eq!(
            err.to_string(),
            "api error (status 400, code 1001): bad request"
        );
    }

    #[test]
    fn api_msg_no_code() {
        let err = ApiError::new(401, None, "unauthorized");
        assert_eq!(err.to_string(), "api error (status 401): unauthorized");
    }
}
