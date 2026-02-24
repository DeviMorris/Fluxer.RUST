use core::fmt;
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelType {
    GuildText,
    Dm,
    GuildVoice,
    GroupDm,
    GuildCategory,
    GuildLinkExtended,
    Unknown(i32),
}

impl ChannelType {
    pub const fn code(self) -> i32 {
        match self {
            Self::GuildText => 0,
            Self::Dm => 1,
            Self::GuildVoice => 2,
            Self::GroupDm => 3,
            Self::GuildCategory => 4,
            Self::GuildLinkExtended => 998,
            Self::Unknown(v) => v,
        }
    }

    pub const fn from_code(code: i32) -> Self {
        match code {
            0 => Self::GuildText,
            1 => Self::Dm,
            2 => Self::GuildVoice,
            3 => Self::GroupDm,
            4 => Self::GuildCategory,
            998 => Self::GuildLinkExtended,
            other => Self::Unknown(other),
        }
    }
}

impl Serialize for ChannelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.code())
    }
}

impl<'de> Deserialize<'de> for ChannelType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_code(i32::deserialize(deserializer)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionOverwriteType {
    Role,
    Member,
    Unknown(i32),
}

impl PermissionOverwriteType {
    pub const fn code(self) -> i32 {
        match self {
            Self::Role => 0,
            Self::Member => 1,
            Self::Unknown(v) => v,
        }
    }

    pub const fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Role,
            1 => Self::Member,
            v => Self::Unknown(v),
        }
    }
}

impl Serialize for PermissionOverwriteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.code())
    }
}

impl<'de> Deserialize<'de> for PermissionOverwriteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_code(i32::deserialize(deserializer)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebhookType {
    Incoming,
    ChannelFollower,
    Application,
    Unknown(i32),
}

impl WebhookType {
    pub const fn code(self) -> i32 {
        match self {
            Self::Incoming => 1,
            Self::ChannelFollower => 2,
            Self::Application => 3,
            Self::Unknown(v) => v,
        }
    }

    pub const fn from_code(code: i32) -> Self {
        match code {
            1 => Self::Incoming,
            2 => Self::ChannelFollower,
            3 => Self::Application,
            v => Self::Unknown(v),
        }
    }
}

impl Serialize for WebhookType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.code())
    }
}

impl<'de> Deserialize<'de> for WebhookType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_code(i32::deserialize(deserializer)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    Twitch,
    YouTube,
    Bot,
    GuildSubscription,
    Unknown(String),
}

impl IntegrationType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Twitch => "twitch",
            Self::YouTube => "youtube",
            Self::Bot => "fluxer",
            Self::GuildSubscription => "guild_subscription",
            Self::Unknown(v) => v.as_str(),
        }
    }
}

impl From<&str> for IntegrationType {
    fn from(value: &str) -> Self {
        match value {
            "twitch" => Self::Twitch,
            "youtube" => Self::YouTube,
            "fluxer" | "discord" => Self::Bot,
            "guild_subscription" => Self::GuildSubscription,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl Serialize for IntegrationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for IntegrationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Bearer,
    Bot,
    Unknown(String),
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
            Self::Bot => "Bot",
            Self::Unknown(v) => v.as_str(),
        }
    }
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "Bearer" => Self::Bearer,
            "Bot" => Self::Bot,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl Serialize for TokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OAuth2Scope {
    ActivitiesRead,
    ActivitiesWrite,
    ApplicationsBuildsRead,
    ApplicationsBuildsUpload,
    ApplicationsCommands,
    ApplicationsCommandsUpdate,
    ApplicationsCommandsPermissionsUpdate,
    ApplicationsEntitlements,
    ApplicationsStoreUpdate,
    Rpc,
    RpcNotificationsRead,
    RpcVoiceWrite,
    RpcVoiceRead,
    RpcActivitiesWrite,
    Guilds,
    GuildsJoin,
    GuildsMembersRead,
    GdmJoin,
    RelationshipsRead,
    RoleConnectionsWrite,
    Identify,
    Email,
    Connections,
    Bot,
    MessagesRead,
    WebhookIncoming,
    Unknown(String),
}

impl OAuth2Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ActivitiesRead => "activities.read",
            Self::ActivitiesWrite => "activities.write",
            Self::ApplicationsBuildsRead => "applications.builds.read",
            Self::ApplicationsBuildsUpload => "applications.builds.upload",
            Self::ApplicationsCommands => "applications.commands",
            Self::ApplicationsCommandsUpdate => "applications.commands.update",
            Self::ApplicationsCommandsPermissionsUpdate => {
                "applications.commands.permissions.update"
            }
            Self::ApplicationsEntitlements => "applications.entitlements",
            Self::ApplicationsStoreUpdate => "applications.store.update",
            Self::Rpc => "rpc",
            Self::RpcNotificationsRead => "rpc.notifications.read",
            Self::RpcVoiceWrite => "rpc.voice.write",
            Self::RpcVoiceRead => "rpc.voice.read",
            Self::RpcActivitiesWrite => "rpc.activities.write",
            Self::Guilds => "guilds",
            Self::GuildsJoin => "guilds.join",
            Self::GuildsMembersRead => "guilds.members.read",
            Self::GdmJoin => "gdm.join",
            Self::RelationshipsRead => "relationships.read",
            Self::RoleConnectionsWrite => "role_connections.write",
            Self::Identify => "identify",
            Self::Email => "email",
            Self::Connections => "connections",
            Self::Bot => "bot",
            Self::MessagesRead => "messages.read",
            Self::WebhookIncoming => "webhook.incoming",
            Self::Unknown(v) => v.as_str(),
        }
    }
}

impl From<&str> for OAuth2Scope {
    fn from(value: &str) -> Self {
        match value {
            "activities.read" => Self::ActivitiesRead,
            "activities.write" => Self::ActivitiesWrite,
            "applications.builds.read" => Self::ApplicationsBuildsRead,
            "applications.builds.upload" => Self::ApplicationsBuildsUpload,
            "applications.commands" => Self::ApplicationsCommands,
            "applications.commands.update" => Self::ApplicationsCommandsUpdate,
            "applications.commands.permissions.update" => {
                Self::ApplicationsCommandsPermissionsUpdate
            }
            "applications.entitlements" => Self::ApplicationsEntitlements,
            "applications.store.update" => Self::ApplicationsStoreUpdate,
            "rpc" => Self::Rpc,
            "rpc.notifications.read" => Self::RpcNotificationsRead,
            "rpc.voice.write" => Self::RpcVoiceWrite,
            "rpc.voice.read" => Self::RpcVoiceRead,
            "rpc.activities.write" => Self::RpcActivitiesWrite,
            "guilds" => Self::Guilds,
            "guilds.join" => Self::GuildsJoin,
            "guilds.members.read" => Self::GuildsMembersRead,
            "gdm.join" => Self::GdmJoin,
            "relationships.read" => Self::RelationshipsRead,
            "role_connections.write" => Self::RoleConnectionsWrite,
            "identify" => Self::Identify,
            "email" => Self::Email,
            "connections" => Self::Connections,
            "bot" => Self::Bot,
            "messages.read" => Self::MessagesRead,
            "webhook.incoming" => Self::WebhookIncoming,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl FromStr for OAuth2Scope {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl fmt::Display for OAuth2Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for OAuth2Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OAuth2Scope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_type_unknown_roundtrip() {
        let value = ChannelType::Unknown(777);
        let json = serde_json::to_string(&value).expect("serialize");
        let back: ChannelType = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, value);
    }

    #[test]
    fn integration_type_unknown_roundtrip() {
        let value: IntegrationType = serde_json::from_str("\"x-custom\"").expect("deserialize");
        assert!(matches!(value, IntegrationType::Unknown(_)));
        let out = serde_json::to_string(&value).expect("serialize");
        assert_eq!(out, "\"x-custom\"");
    }

    #[test]
    fn scope_known_and_unknown() {
        let known: OAuth2Scope = "identify".into();
        let unknown: OAuth2Scope = "my.scope".into();
        assert_eq!(known.as_str(), "identify");
        assert_eq!(unknown.as_str(), "my.scope");
    }
}
