/// Snowflake ID — 64-bit unsigned integer serialized as string.
///
/// Fluxer uses Twitter Snowflakes with epoch `1420070400000`
/// (first second of 2015 UTC).
pub type Snowflake = String;

/// Fluxer epoch in milliseconds (2015-01-01 00:00:00 UTC).
pub const FLUXER_EPOCH: u64 = 1420070400000;

/// Extract the Unix timestamp (ms) from a snowflake string.
///
/// Returns `None` if the string is not a valid u64.
pub fn snowflake_timestamp(id: &str) -> Option<u64> {
    let n: u64 = id.parse().ok()?;
    Some((n >> 22) + FLUXER_EPOCH)
}

/// Deconstruct a snowflake into `(timestamp_ms, worker, process, increment)`.
///
/// Returns `None` if the string is not a valid u64.
pub fn snowflake_deconstruct(id: &str) -> Option<(u64, u64, u64, u64)> {
    let n: u64 = id.parse().ok()?;
    Some((
        (n >> 22) + FLUXER_EPOCH,
        (n >> 17) & 0x1F,
        (n >> 12) & 0x1F,
        n & 0xFFF,
    ))
}
