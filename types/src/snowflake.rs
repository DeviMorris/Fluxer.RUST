pub type Snowflake = String;

pub const FLUXER_EPOCH: u64 = 1420070400000;

pub fn snowflake_timestamp(id: &str) -> Option<u64> {
    let n: u64 = id.parse().ok()?;
    Some((n >> 22) + FLUXER_EPOCH)
}

pub fn snowflake_deconstruct(id: &str) -> Option<(u64, u64, u64, u64)> {
    let n: u64 = id.parse().ok()?;
    Some((
        (n >> 22) + FLUXER_EPOCH,
        (n >> 17) & 0x1F,
        (n >> 12) & 0x1F,
        n & 0xFFF,
    ))
}
