use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::enums::{ChannelType, IntegrationType, PermissionOverwriteType, WebhookType};
use crate::flags::Permissions;
use crate::id::Snowflake;

fn parse_i32_tag(value: &Value, field: &str) -> Option<i32> {
    match value.get(field) {
        Some(Value::Number(n)) => n.as_i64().and_then(|v| i32::try_from(v).ok()),
        Some(Value::String(s)) => s.parse::<i32>().ok(),
        _ => None,
    }
}

fn parse_string_tag(value: &Value, field: &str) -> Option<String> {
    match value.get(field) {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Number(n)) => Some(n.to_string()),
        _ => None,
    }
}

fn serialize_with_tag<T, S>(
    value: &T,
    field: &'static str,
    tag: Value,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut v = serde_json::to_value(value).map_err(S::Error::custom)?;
    if let Value::Object(ref mut map) = v {
        map.insert(field.to_string(), tag);
    }
    v.serialize(serializer)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Channel {
    GuildText(GuildTextChannel),
    Dm(DmChannel),
    GuildVoice(GuildVoiceChannel),
    GroupDm(GroupDmChannel),
    GuildCategory(GuildCategoryChannel),
    GuildLinkExtended(GuildLinkExtendedChannel),
    Unknown {
        kind: Option<ChannelType>,
        raw: Value,
    },
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let kind = parse_i32_tag(&raw, "type").map(ChannelType::from_code);
        match kind {
            Some(ChannelType::GuildText) => serde_json::from_value::<GuildTextChannel>(raw.clone())
                .map(Self::GuildText)
                .map_err(D::Error::custom),
            Some(ChannelType::Dm) => serde_json::from_value::<DmChannel>(raw.clone())
                .map(Self::Dm)
                .map_err(D::Error::custom),
            Some(ChannelType::GuildVoice) => {
                serde_json::from_value::<GuildVoiceChannel>(raw.clone())
                    .map(Self::GuildVoice)
                    .map_err(D::Error::custom)
            }
            Some(ChannelType::GroupDm) => serde_json::from_value::<GroupDmChannel>(raw.clone())
                .map(Self::GroupDm)
                .map_err(D::Error::custom),
            Some(ChannelType::GuildCategory) => {
                serde_json::from_value::<GuildCategoryChannel>(raw.clone())
                    .map(Self::GuildCategory)
                    .map_err(D::Error::custom)
            }
            Some(ChannelType::GuildLinkExtended) => {
                serde_json::from_value::<GuildLinkExtendedChannel>(raw.clone())
                    .map(Self::GuildLinkExtended)
                    .map_err(D::Error::custom)
            }
            _ => Ok(Self::Unknown { kind, raw }),
        }
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::GuildText(v) => serialize_with_tag(
                v,
                "type",
                Value::from(ChannelType::GuildText.code()),
                serializer,
            ),
            Self::Dm(v) => {
                serialize_with_tag(v, "type", Value::from(ChannelType::Dm.code()), serializer)
            }
            Self::GuildVoice(v) => serialize_with_tag(
                v,
                "type",
                Value::from(ChannelType::GuildVoice.code()),
                serializer,
            ),
            Self::GroupDm(v) => serialize_with_tag(
                v,
                "type",
                Value::from(ChannelType::GroupDm.code()),
                serializer,
            ),
            Self::GuildCategory(v) => serialize_with_tag(
                v,
                "type",
                Value::from(ChannelType::GuildCategory.code()),
                serializer,
            ),
            Self::GuildLinkExtended(v) => serialize_with_tag(
                v,
                "type",
                Value::from(ChannelType::GuildLinkExtended.code()),
                serializer,
            ),
            Self::Unknown { raw, .. } => raw.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Integration {
    Twitch(TwitchIntegration),
    YouTube(YouTubeIntegration),
    Bot(BotIntegration),
    GuildSubscription(GuildSubscriptionIntegration),
    Unknown {
        kind: Option<IntegrationType>,
        raw: Value,
    },
}

impl<'de> Deserialize<'de> for Integration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let kind = parse_string_tag(&raw, "type").map(|v| IntegrationType::from(v.as_str()));
        match kind {
            Some(IntegrationType::Twitch) => {
                serde_json::from_value::<TwitchIntegration>(raw.clone())
                    .map(Self::Twitch)
                    .map_err(D::Error::custom)
            }
            Some(IntegrationType::YouTube) => {
                serde_json::from_value::<YouTubeIntegration>(raw.clone())
                    .map(Self::YouTube)
                    .map_err(D::Error::custom)
            }
            Some(IntegrationType::Bot) => serde_json::from_value::<BotIntegration>(raw.clone())
                .map(Self::Bot)
                .map_err(D::Error::custom),
            Some(IntegrationType::GuildSubscription) => {
                serde_json::from_value::<GuildSubscriptionIntegration>(raw.clone())
                    .map(Self::GuildSubscription)
                    .map_err(D::Error::custom)
            }
            _ => Ok(Self::Unknown { kind, raw }),
        }
    }
}

impl Serialize for Integration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Twitch(v) => serialize_with_tag(
                v,
                "type",
                Value::from(IntegrationType::Twitch.as_str()),
                serializer,
            ),
            Self::YouTube(v) => serialize_with_tag(
                v,
                "type",
                Value::from(IntegrationType::YouTube.as_str()),
                serializer,
            ),
            Self::Bot(v) => serialize_with_tag(
                v,
                "type",
                Value::from(IntegrationType::Bot.as_str()),
                serializer,
            ),
            Self::GuildSubscription(v) => serialize_with_tag(
                v,
                "type",
                Value::from(IntegrationType::GuildSubscription.as_str()),
                serializer,
            ),
            Self::Unknown { raw, .. } => raw.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionOverwrite {
    Role(RolePermissionOverwrite),
    Member(MemberPermissionOverwrite),
    Unknown {
        kind: Option<PermissionOverwriteType>,
        raw: Value,
    },
}

impl<'de> Deserialize<'de> for PermissionOverwrite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let kind = parse_i32_tag(&raw, "type").map(PermissionOverwriteType::from_code);
        match kind {
            Some(PermissionOverwriteType::Role) => {
                serde_json::from_value::<RolePermissionOverwrite>(raw.clone())
                    .map(Self::Role)
                    .map_err(D::Error::custom)
            }
            Some(PermissionOverwriteType::Member) => {
                serde_json::from_value::<MemberPermissionOverwrite>(raw.clone())
                    .map(Self::Member)
                    .map_err(D::Error::custom)
            }
            _ => Ok(Self::Unknown { kind, raw }),
        }
    }
}

impl Serialize for PermissionOverwrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Role(v) => serialize_with_tag(
                v,
                "type",
                Value::from(PermissionOverwriteType::Role.code()),
                serializer,
            ),
            Self::Member(v) => serialize_with_tag(
                v,
                "type",
                Value::from(PermissionOverwriteType::Member.code()),
                serializer,
            ),
            Self::Unknown { raw, .. } => raw.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Webhook {
    Incoming(IncomingWebhook),
    ChannelFollower(ChannelFollowerWebhook),
    Application(ApplicationWebhook),
    Unknown {
        kind: Option<WebhookType>,
        raw: Value,
    },
}

impl<'de> Deserialize<'de> for Webhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let kind = parse_i32_tag(&raw, "type").map(WebhookType::from_code);
        match kind {
            Some(WebhookType::Incoming) => serde_json::from_value::<IncomingWebhook>(raw.clone())
                .map(Self::Incoming)
                .map_err(D::Error::custom),
            Some(WebhookType::ChannelFollower) => {
                serde_json::from_value::<ChannelFollowerWebhook>(raw.clone())
                    .map(Self::ChannelFollower)
                    .map_err(D::Error::custom)
            }
            Some(WebhookType::Application) => {
                serde_json::from_value::<ApplicationWebhook>(raw.clone())
                    .map(Self::Application)
                    .map_err(D::Error::custom)
            }
            _ => Ok(Self::Unknown { kind, raw }),
        }
    }
}

impl Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Incoming(v) => serialize_with_tag(
                v,
                "type",
                Value::from(WebhookType::Incoming.code()),
                serializer,
            ),
            Self::ChannelFollower(v) => serialize_with_tag(
                v,
                "type",
                Value::from(WebhookType::ChannelFollower.code()),
                serializer,
            ),
            Self::Application(v) => serialize_with_tag(
                v,
                "type",
                Value::from(WebhookType::Application.code()),
                serializer,
            ),
            Self::Unknown { raw, .. } => raw.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildTextChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DmChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub last_message_id: Option<Snowflake>,
    #[serde(default)]
    pub recipients: Vec<PartialUser>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupDmChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub owner_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildVoiceChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildCategoryChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildLinkExtendedChannel {
    pub id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RolePermissionOverwrite {
    pub id: Snowflake,
    #[serde(default)]
    pub allow: Permissions,
    #[serde(default)]
    pub deny: Permissions,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberPermissionOverwrite {
    pub id: Snowflake,
    #[serde(default)]
    pub allow: Permissions,
    #[serde(default)]
    pub deny: Permissions,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwitchIntegration {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YouTubeIntegration {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotIntegration {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub scopes: Vec<crate::enums::OAuth2Scope>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildSubscriptionIntegration {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncomingWebhook {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelFollowerWebhook {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationWebhook {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartialUser {
    pub id: Snowflake,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_dispatch_known() {
        let raw = r#"{"type":0,"id":"1","name":"general"}"#;
        let channel: Channel = serde_json::from_str(raw).expect("channel decode");
        match channel {
            Channel::GuildText(v) => assert_eq!(v.id, Snowflake::new(1)),
            _ => panic!("unexpected channel variant"),
        }
    }

    #[test]
    fn channel_dispatch_unknown_preserves_raw() {
        let raw = r#"{"type":777,"id":"1","foo":"bar"}"#;
        let channel: Channel = serde_json::from_str(raw).expect("channel decode");
        match channel {
            Channel::Unknown { kind, raw } => {
                assert!(matches!(kind, Some(ChannelType::Unknown(777))));
                assert_eq!(raw.get("foo").and_then(Value::as_str), Some("bar"));
            }
            _ => panic!("expected unknown channel variant"),
        }
    }

    #[test]
    fn permission_overwrite_dispatch_known() {
        let raw = r#"{"type":0,"id":"1","allow":"1024","deny":"0"}"#;
        let overwrite: PermissionOverwrite = serde_json::from_str(raw).expect("decode");
        match overwrite {
            PermissionOverwrite::Role(v) => assert_eq!(v.allow, Permissions::VIEW_CHANNEL),
            _ => panic!("unexpected overwrite variant"),
        }
    }
}
