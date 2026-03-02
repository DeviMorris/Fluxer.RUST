use std::time::{Duration, SystemTime, UNIX_EPOCH};

const FLUXER_EPOCH: u64 = 1_420_070_400_000;

pub struct SnowflakeUtil;

impl SnowflakeUtil {
    pub fn date_from_snowflake(id: &str) -> Option<SystemTime> {
        let n: u64 = id.parse().ok()?;
        let ms = (n >> 22) + FLUXER_EPOCH;
        Some(UNIX_EPOCH + Duration::from_millis(ms))
    }

    pub fn timestamp_ms_from_snowflake(id: &str) -> Option<u64> {
        let n: u64 = id.parse().ok()?;
        Some((n >> 22) + FLUXER_EPOCH)
    }

    pub fn snowflake_from_timestamp(ms: u64) -> String {
        let epoch_ms = ms.saturating_sub(FLUXER_EPOCH);
        let sf = epoch_ms << 22;
        sf.to_string()
    }

    pub fn is_valid(id: &str) -> bool {
        id.parse::<u64>().is_ok() && !id.is_empty()
    }
}
