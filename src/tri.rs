use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nullable<T> {
    Null,
    Value(T),
}

impl<T> Nullable<T> {
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_ref(&self) -> Nullable<&T> {
        match self {
            Self::Null => Nullable::Null,
            Self::Value(v) => Nullable::Value(v),
        }
    }

    pub fn map<U, F>(self, mut f: F) -> Nullable<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Self::Null => Nullable::Null,
            Self::Value(v) => Nullable::Value(f(v)),
        }
    }
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Self::Null
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::Value(v),
            None => Self::Null,
        }
    }
}

impl<T> From<Nullable<T>> for Option<T> {
    fn from(value: Nullable<T>) -> Self {
        match value {
            Nullable::Null => None,
            Nullable::Value(v) => Some(v),
        }
    }
}

impl<T> Serialize for Nullable<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => serializer.serialize_some(v),
        }
    }
}

impl<'de, T> Deserialize<'de> for Nullable<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe = Option::<T>::deserialize(deserializer)?;
        Ok(match maybe {
            Some(v) => Self::Value(v),
            None => Self::Null,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch<T> {
    Omitted,
    Null,
    Value(T),
}

impl<T> Patch<T> {
    pub const fn omitted() -> Self {
        Self::Omitted
    }

    pub fn value(value: T) -> Self {
        Self::Value(value)
    }

    pub const fn null() -> Self {
        Self::Null
    }

    pub const fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }

    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_ref(&self) -> Patch<&T> {
        match self {
            Self::Omitted => Patch::Omitted,
            Self::Null => Patch::Null,
            Self::Value(v) => Patch::Value(v),
        }
    }
}

impl<T> Default for Patch<T> {
    fn default() -> Self {
        Self::Omitted
    }
}

impl<T> Serialize for Patch<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Omitted => Err(S::Error::custom(
                "Patch::Omitted cannot be serialized directly; use #[serde(skip_serializing_if = \"Patch::is_omitted\")]",
            )),
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let maybe = Option::<T>::deserialize(deserializer)?;
        Ok(match maybe {
            Some(v) => Self::Value(v),
            None => Self::Null,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct PatchHolder {
        #[serde(default, skip_serializing_if = "Patch::is_omitted")]
        name: Patch<String>,
    }

    #[test]
    fn patch_omitted_skips_field() {
        let value = PatchHolder {
            name: Patch::Omitted,
        };
        let json = serde_json::to_string(&value).expect("serialize");
        assert_eq!(json, "{}");
    }

    #[test]
    fn patch_null_serializes_as_null() {
        let value = PatchHolder { name: Patch::Null };
        let json = serde_json::to_string(&value).expect("serialize");
        assert_eq!(json, r#"{"name":null}"#);
    }

    #[test]
    fn patch_deserialize_missing_as_omitted() {
        let value: PatchHolder = serde_json::from_str("{}").expect("deserialize");
        assert!(value.name.is_omitted());
    }

    #[test]
    fn patch_deserialize_null() {
        let value: PatchHolder = serde_json::from_str(r#"{"name":null}"#).expect("deserialize");
        assert!(value.name.is_null());
    }
}
