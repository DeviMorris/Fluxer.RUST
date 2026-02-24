use bitflags::bitflags;
use core::fmt;
use serde::de::{Error as DeError, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

macro_rules! impl_numeric_serde_flags {
    ($name:ident, $repr:ty) => {
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u64(self.bits() as u64)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let raw = parse_u64_any(deserializer, concat!(stringify!($name), " as integer"))?;
                let bits = <$repr>::try_from(raw).map_err(D::Error::custom)?;
                Ok(Self::from_bits_retain(bits))
            }
        }
    };
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Permissions: u64 {
        const CREATE_INSTANT_INVITE                = 1u64 << 0;
        const KICK_MEMBERS                         = 1u64 << 1;
        const BAN_MEMBERS                          = 1u64 << 2;
        const ADMINISTRATOR                        = 1u64 << 3;
        const MANAGE_CHANNELS                      = 1u64 << 4;
        const MANAGE_GUILD                         = 1u64 << 5;
        const ADD_REACTIONS                        = 1u64 << 6;
        const VIEW_AUDIT_LOG                       = 1u64 << 7;
        const PRIORITY_SPEAKER                     = 1u64 << 8;
        const STREAM                               = 1u64 << 9;
        const VIEW_CHANNEL                         = 1u64 << 10;
        const SEND_MESSAGES                        = 1u64 << 11;
        const SEND_TTS_MESSAGES                    = 1u64 << 12;
        const MANAGE_MESSAGES                      = 1u64 << 13;
        const EMBED_LINKS                          = 1u64 << 14;
        const ATTACH_FILES                         = 1u64 << 15;
        const READ_MESSAGE_HISTORY                 = 1u64 << 16;
        const MENTION_EVERYONE                     = 1u64 << 17;
        const USE_EXTERNAL_EMOJIS                  = 1u64 << 18;
        const VIEW_GUILD_INSIGHTS                  = 1u64 << 19;
        const CONNECT                              = 1u64 << 20;
        const SPEAK                                = 1u64 << 21;
        const MUTE_MEMBERS                         = 1u64 << 22;
        const DEAFEN_MEMBERS                       = 1u64 << 23;
        const MOVE_MEMBERS                         = 1u64 << 24;
        const USE_VAD                              = 1u64 << 25;
        const CHANGE_NICKNAME                      = 1u64 << 26;
        const MANAGE_NICKNAMES                     = 1u64 << 27;
        const MANAGE_ROLES                         = 1u64 << 28;
        const MANAGE_WEBHOOKS                      = 1u64 << 29;
        const MANAGE_GUILD_EXPRESSIONS             = 1u64 << 30;
        const USE_APPLICATION_COMMANDS             = 1u64 << 31;
        const REQUEST_TO_SPEAK                     = 1u64 << 32;
        const MANAGE_EVENTS                        = 1u64 << 33;
        const MANAGE_THREADS                       = 1u64 << 34;
        const CREATE_PUBLIC_THREADS                = 1u64 << 35;
        const CREATE_PRIVATE_THREADS               = 1u64 << 36;
        const USE_EXTERNAL_STICKERS                = 1u64 << 37;
        const SEND_MESSAGES_IN_THREADS             = 1u64 << 38;
        const USE_EMBEDDED_ACTIVITIES              = 1u64 << 39;
        const MODERATE_MEMBERS                     = 1u64 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS  = 1u64 << 41;
        const USE_SOUNDBOARD                       = 1u64 << 42;
        const CREATE_GUILD_EXPRESSIONS             = 1u64 << 43;
        const CREATE_EVENTS                        = 1u64 << 44;
        const USE_EXTERNAL_SOUNDS                  = 1u64 << 45;
        const SEND_VOICE_MESSAGES                  = 1u64 << 46;
        const SEND_POLLS                           = 1u64 << 48;
        const USE_EXTERNAL_APPS                    = 1u64 << 49;
        const PIN_MESSAGES                         = 1u64 << 50;
        const BYPASS_SLOWMODE                      = 1u64 << 51;
    }
}

impl Permissions {
    pub fn added(self, bits: Self) -> Self {
        self | bits
    }

    pub fn removed(self, bits: Self) -> Self {
        self & !bits
    }

    pub fn has_all(self, bits: Self) -> bool {
        self.contains(bits)
    }

    pub fn missing_any(self, bits: Self) -> bool {
        !self.contains(bits)
    }

    pub const fn all_text() -> Self {
        Self::from_bits_retain(
            Self::VIEW_CHANNEL.bits()
                | Self::SEND_MESSAGES.bits()
                | Self::SEND_TTS_MESSAGES.bits()
                | Self::MANAGE_MESSAGES.bits()
                | Self::EMBED_LINKS.bits()
                | Self::ATTACH_FILES.bits()
                | Self::READ_MESSAGE_HISTORY.bits()
                | Self::MENTION_EVERYONE.bits()
                | Self::SEND_VOICE_MESSAGES.bits()
                | Self::SEND_POLLS.bits()
                | Self::USE_EXTERNAL_APPS.bits()
                | Self::PIN_MESSAGES.bits()
                | Self::BYPASS_SLOWMODE.bits(),
        )
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.bits().to_string())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = parse_u64_any(deserializer, "permissions as integer string or number")?;
        Ok(Self::from_bits_retain(raw))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct MessageFlags: u32 {
        const CROSSPOSTED                            = 1u32 << 0;
        const IS_CROSSPOST                           = 1u32 << 1;
        const SUPPRESS_EMBEDS                        = 1u32 << 2;
        const SOURCE_MESSAGE_DELETED                 = 1u32 << 3;
        const URGENT                                 = 1u32 << 4;
        const HAS_THREAD                             = 1u32 << 5;
        const EPHEMERAL                              = 1u32 << 6;
        const LOADING                                = 1u32 << 7;
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1u32 << 8;
        const SUPPRESS_NOTIFICATIONS                 = 1u32 << 12;
        const IS_VOICE_MESSAGE                       = 1u32 << 13;
        const HAS_SNAPSHOT                           = 1u32 << 14;
        const IS_COMPONENTS_V2                       = 1u32 << 15;
    }
}
impl_numeric_serde_flags!(MessageFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct UserFlags: u32 {
        const FLUXER_EMPLOYEE                  = 1u32 << 0;
        const PARTNERED_SERVER_OWNER           = 1u32 << 1;
        const HYPESQUAD_EVENTS                 = 1u32 << 2;
        const BUG_HUNTER_LEVEL_1               = 1u32 << 3;
        const HOUSE_BRAVERY                    = 1u32 << 6;
        const HOUSE_BRILLIANCE                 = 1u32 << 7;
        const HOUSE_BALANCE                    = 1u32 << 8;
        const EARLY_SUPPORTER                  = 1u32 << 9;
        const TEAM_USER                        = 1u32 << 10;
        const BUG_HUNTER_LEVEL_2               = 1u32 << 14;
        const VERIFIED_BOT                     = 1u32 << 16;
        const EARLY_VERIFIED_BOT_DEVELOPER     = 1u32 << 17;
        const FLUXER_CERTIFIED_MODERATOR       = 1u32 << 18;
        const BOT_HTTP_INTERACTIONS            = 1u32 << 19;
    }
}
impl_numeric_serde_flags!(UserFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct MemberFlags: u32 {
        const DID_REJOIN                       = 1u32 << 0;
        const COMPLETED_ONBOARDING            = 1u32 << 1;
        const BYPASSES_VERIFICATION           = 1u32 << 2;
        const STARTED_ONBOARDING              = 1u32 << 3;
        const IS_GUEST                        = 1u32 << 4;
        const STARTED_HOME_ACTIONS            = 1u32 << 5;
        const COMPLETED_HOME_ACTIONS          = 1u32 << 6;
        const AUTOMOD_QUARANTINED_USERNAME    = 1u32 << 7;
        const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1u32 << 9;
        const AUTOMOD_QUARANTINED_GUILD_TAG   = 1u32 << 10;
    }
}
impl_numeric_serde_flags!(MemberFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct ApplicationFlags: u32 {
        const AUTO_MODERATION_RULE_CREATE_BADGE = 1u32 << 6;
        const GATEWAY_PRESENCE                  = 1u32 << 12;
        const GATEWAY_PRESENCE_LIMITED          = 1u32 << 13;
        const GATEWAY_GUILD_MEMBERS             = 1u32 << 14;
        const GATEWAY_GUILD_MEMBER_LIMITED      = 1u32 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT  = 1u32 << 16;
        const EMBEDDED                          = 1u32 << 17;
        const GATEWAY_MESSAGE_CONTENT           = 1u32 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED   = 1u32 << 19;
        const APPLICATION_COMMAND_BADGE         = 1u32 << 23;
    }
}
impl_numeric_serde_flags!(ApplicationFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct SystemChannelFlags: u32 {
        const SUPPRESS_JOIN_NOTIFICATIONS                            = 1u32 << 0;
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS                         = 1u32 << 1;
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS                  = 1u32 << 2;
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES                     = 1u32 << 3;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS      = 1u32 << 4;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1u32 << 5;
    }
}
impl_numeric_serde_flags!(SystemChannelFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct FileFlags: u32 {
        const SPOILER = 1u32 << 0;
    }
}
impl_numeric_serde_flags!(FileFlags, u32);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct ChannelFlags: u32 {
        const PINNED                       = 1u32 << 1;
        const REQUIRE_TAG                  = 1u32 << 4;
        const HIDE_MEDIA_DOWNLOAD_OPTIONS  = 1u32 << 15;
    }
}
impl_numeric_serde_flags!(ChannelFlags, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissions_serialize_as_string() {
        let p = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;
        let json = serde_json::to_string(&p).expect("serialize permissions");
        let expected = (Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES)
            .bits()
            .to_string();
        assert_eq!(json, format!("\"{expected}\""));
    }

    #[test]
    fn permissions_parse_from_string() {
        let parsed: Permissions =
            serde_json::from_str("\"1024\"").expect("deserialize permissions");
        assert!(parsed.has_all(Permissions::VIEW_CHANNEL));
    }

    #[test]
    fn message_flags_roundtrip() {
        let flags = MessageFlags::SUPPRESS_EMBEDS | MessageFlags::IS_VOICE_MESSAGE;
        let json = serde_json::to_string(&flags).expect("serialize message flags");
        let back: MessageFlags = serde_json::from_str(&json).expect("deserialize message flags");
        assert_eq!(back, flags);
    }
}
