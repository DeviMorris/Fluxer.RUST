use chrono::{DateTime, Utc};
use core::fmt;
use core::str::FromStr;
use serde::de::{Error as DeError, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const SNOWFLAKE_EPOCH_MILLIS: u64 = 1_420_070_400_000;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Snowflake(u64);

impl Snowflake {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub const fn timestamp_part(self) -> u64 {
        self.0 >> 22
    }

    pub const fn timestamp_millis(self) -> u64 {
        self.timestamp_part() + SNOWFLAKE_EPOCH_MILLIS
    }

    pub fn created_at(self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp_millis(self.timestamp_millis() as i64)
    }

    pub const fn worker_id(self) -> u8 {
        ((self.0 >> 17) & 0x1F) as u8
    }

    pub const fn process_id(self) -> u8 {
        ((self.0 >> 12) & 0x1F) as u8
    }

    pub const fn increment(self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Snowflake> for u64 {
    fn from(value: Snowflake) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowflakeParseError {
    Empty,
    Negative,
    Invalid,
}

impl fmt::Display for SnowflakeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "snowflake is empty"),
            Self::Negative => write!(f, "snowflake cannot be negative"),
            Self::Invalid => write!(f, "invalid snowflake"),
        }
    }
}

impl std::error::Error for SnowflakeParseError {}

impl FromStr for Snowflake {
    type Err = SnowflakeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim();
        if value.is_empty() {
            return Err(SnowflakeParseError::Empty);
        }
        if value.starts_with('-') {
            return Err(SnowflakeParseError::Negative);
        }

        let raw = value
            .parse::<u64>()
            .map_err(|_| SnowflakeParseError::Invalid)?;
        Ok(Self(raw))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeVisitor;

        impl Visitor<'_> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("snowflake string or non-negative integer")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Snowflake::from_str(value).map_err(E::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Snowflake::new(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value < 0 {
                    return Err(E::invalid_value(Unexpected::Signed(value), &self));
                }
                Ok(Snowflake::new(value as u64))
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_format() {
        let id: Snowflake = "175928847299117063".parse().expect("valid snowflake");
        assert_eq!(id.to_string(), "175928847299117063");
    }

    #[test]
    fn parts() {
        let id = Snowflake::new(175928847299117063);
        assert_eq!(id.worker_id(), 1);
        assert_eq!(id.process_id(), 0);
        assert_eq!(id.increment(), 7);
    }

    #[test]
    fn ser_string() {
        let id = Snowflake::new(42);
        let json = serde_json::to_string(&id).expect("serialize snowflake");
        assert_eq!(json, "\"42\"");
    }

    #[test]
    fn de_num_or_str() {
        let from_str: Snowflake = serde_json::from_str("\"42\"").expect("from string");
        let from_num: Snowflake = serde_json::from_str("42").expect("from number");
        assert_eq!(from_str, Snowflake::new(42));
        assert_eq!(from_num, Snowflake::new(42));
    }
}
