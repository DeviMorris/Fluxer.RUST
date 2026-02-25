use chrono::{DateTime, Utc};
use core::fmt;
use core::str::FromStr;
use serde::de::{Error as DeError, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serializer};
use std::time::Duration;

use crate::enums::OAuth2Scope;
use crate::id::Snowflake;

fn parse_u64_any<'de, D>(deserializer: D, expected: &'static str) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {
        expected: &'static str,
    }

    impl Visitor<'_> for V {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.expected)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            if value < 0 {
                return Err(E::invalid_value(Unexpected::Signed(value), &self));
            }
            Ok(value as u64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            value.parse::<u64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(V { expected })
}

fn parse_i64_any<'de, D>(deserializer: D, expected: &'static str) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {
        expected: &'static str,
    }

    impl Visitor<'_> for V {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.expected)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            i64::try_from(value).map_err(E::custom)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            value.parse::<i64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(V { expected })
}

pub mod snowflake_string {
    use super::*;

    pub fn serialize<S>(value: &Snowflake, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Snowflake, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = super::parse_u64_any(deserializer, "snowflake string or integer")?;
        Ok(Snowflake::new(raw))
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<Snowflake>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(id) => serializer.serialize_some(&id.to_string()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Snowflake>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => {
                    Snowflake::from_str(&s).map(Some).map_err(D::Error::custom)
                }
                Some(serde_json::Value::Number(n)) => n
                    .as_u64()
                    .map(Snowflake::new)
                    .map(Some)
                    .ok_or_else(|| D::Error::custom("invalid snowflake number")),
                Some(_) => Err(D::Error::custom("invalid snowflake value")),
            }
        }
    }
}

pub mod u64_string {
    use super::*;

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        super::parse_u64_any(deserializer, "u64 string or integer")
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(v) => serializer.serialize_some(&v.to_string()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => {
                    s.parse::<u64>().map(Some).map_err(D::Error::custom)
                }
                Some(serde_json::Value::Number(n)) => n
                    .as_u64()
                    .ok_or_else(|| D::Error::custom("invalid u64 number"))
                    .map(Some),
                Some(_) => Err(D::Error::custom("invalid u64 value")),
            }
        }
    }
}

pub mod i64_string {
    use super::*;

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        super::parse_i64_any(deserializer, "i64 string or integer")
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(v) => serializer.serialize_some(&v.to_string()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => {
                    s.parse::<i64>().map(Some).map_err(D::Error::custom)
                }
                Some(serde_json::Value::Number(n)) => n
                    .as_i64()
                    .ok_or_else(|| D::Error::custom("invalid i64 number"))
                    .map(Some),
                Some(_) => Err(D::Error::custom("invalid i64 value")),
            }
        }
    }
}

pub mod duration_seconds {
    use super::*;

    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(value.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = super::parse_u64_any(deserializer, "duration seconds as integer or string")?;
        Ok(Duration::from_secs(secs))
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(v) => serializer.serialize_some(&v.as_secs()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => s
                    .parse::<u64>()
                    .map(Duration::from_secs)
                    .map(Some)
                    .map_err(D::Error::custom),
                Some(serde_json::Value::Number(n)) => n
                    .as_u64()
                    .map(Duration::from_secs)
                    .map(Some)
                    .ok_or_else(|| D::Error::custom("invalid duration number")),
                Some(_) => Err(D::Error::custom("invalid duration value")),
            }
        }
    }
}

pub mod unix_millis {
    use super::*;

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(value.timestamp_millis())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = super::parse_i64_any(deserializer, "unix millis timestamp")?;
        DateTime::<Utc>::from_timestamp_millis(millis)
            .ok_or_else(|| D::Error::custom("invalid unix millis timestamp"))
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(v) => serializer.serialize_some(&v.timestamp_millis()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => {
                    let millis = s.parse::<i64>().map_err(D::Error::custom)?;
                    DateTime::<Utc>::from_timestamp_millis(millis)
                        .ok_or_else(|| D::Error::custom("invalid unix millis"))
                        .map(Some)
                }
                Some(serde_json::Value::Number(n)) => {
                    let millis = n
                        .as_i64()
                        .ok_or_else(|| D::Error::custom("invalid unix millis number"))?;
                    DateTime::<Utc>::from_timestamp_millis(millis)
                        .ok_or_else(|| D::Error::custom("invalid unix millis"))
                        .map(Some)
                }
                Some(_) => Err(D::Error::custom("invalid unix millis value")),
            }
        }
    }
}

pub mod nonce_string_or_int {
    use super::*;

    pub fn serialize<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl Visitor<'_> for V {
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("nonce as string or integer")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(value.to_owned())
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(value.to_string())
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(value.to_string())
            }
        }

        deserializer.deserialize_any(V)
    }

    pub mod opt {
        use super::*;

        pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(v) => serializer.serialize_some(v),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let maybe = Option::<serde_json::Value>::deserialize(deserializer)?;
            match maybe {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(s)) => Ok(Some(s)),
                Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
                Some(_) => Err(D::Error::custom("invalid nonce value")),
            }
        }
    }
}

pub mod scopes_space_delimited {
    use super::*;

    pub fn serialize<S>(value: &[OAuth2Scope], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let joined = value
            .iter()
            .map(OAuth2Scope::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        serializer.serialize_str(&joined)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<OAuth2Scope>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        if raw.trim().is_empty() {
            return Ok(Vec::new());
        }
        Ok(raw.split_whitespace().map(OAuth2Scope::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct IdHolder {
        #[serde(with = "snowflake_string")]
        id: Snowflake,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct U64StringHolder {
        #[serde(with = "u64_string")]
        value: u64,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct NonceHolder {
        #[serde(with = "nonce_string_or_int")]
        nonce: String,
    }

    #[test]
    fn snowflake_roundtrip() {
        let src = IdHolder {
            id: Snowflake::new(123),
        };
        let json = serde_json::to_string(&src).expect("serialize");
        assert_eq!(json, r#"{"id":"123"}"#);
        let back: IdHolder = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, src);
    }

    #[test]
    fn u64_str_roundtrip() {
        let src = U64StringHolder { value: 42 };
        let json = serde_json::to_string(&src).expect("serialize");
        assert_eq!(json, r#"{"value":"42"}"#);
        let back: U64StringHolder = serde_json::from_str(r#"{"value":42}"#).expect("deserialize");
        assert_eq!(back, src);
    }

    #[test]
    fn nonce_number() {
        let parsed: NonceHolder = serde_json::from_str(r#"{"nonce":123}"#).expect("nonce");
        assert_eq!(parsed.nonce, "123");
    }
}
