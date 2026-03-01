/// Unified error type for fluxer-core.
///
/// Covers client lifecycle errors, cache misses, and wraps
/// REST / WebSocket errors from lower-level crates.
///
/// # Errors
/// Returns specific variants for common failure modes:
/// - `ClientNotReady` — method called before `READY` event
/// - `*NotFound` — entity not in cache and API returned 404
/// - `Api` / `Http` / `RateLimit` — forwarded from `fluxer-rest`
///
/// # Examples
/// ```rust,ignore
/// match client.channels().fetch("123").await {
///     Err(Error::ChannelNotFound(id)) => eprintln!("not found: {id}"),
///     Err(Error::Api(e)) => eprintln!("API: {e}"),
///     Ok(ch) => println!("{:?}", ch),
///     _ => {}
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("client not ready")]
    ClientNotReady,

    #[error("invalid token")]
    InvalidToken,

    #[error("already logged in")]
    AlreadyLoggedIn,

    #[error("channel {0} not found")]
    ChannelNotFound(String),

    #[error("message {0} not found")]
    MessageNotFound(String),

    #[error("guild {0} not found")]
    GuildNotFound(String),

    #[error("member {0} not found")]
    MemberNotFound(String),

    #[error("role {0} not found")]
    RoleNotFound(String),

    #[error("emoji {0} not in guild {1}")]
    EmojiNotInGuild(String, String),

    #[error("emoji {0} not found")]
    EmojiNotFound(String),

    #[error("webhook token required to send")]
    WebhookTokenRequired,

    #[error("API error: {0}")]
    Api(#[from] fluxer_rest::FluxerApiError),

    #[error("HTTP error: {0}")]
    Http(#[from] fluxer_rest::HttpError),

    #[error("rate limited: {0}")]
    RateLimit(#[from] fluxer_rest::RateLimitError),

    #[error("REST error: {0}")]
    Rest(#[from] fluxer_rest::RestError),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
