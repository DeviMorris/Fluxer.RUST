use crate::error::{ProtocolError, Result};
use crate::gateway::GatewayEvent;
use crate::id::Snowflake;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct DispatchEnvelope {
    pub seq: u64,
    pub event: DispatchEvent,
}

#[derive(Debug, Clone)]
pub enum DispatchEvent {
    Ready(ReadyPayload),
    Resumed(ResumedPayload),
    ChannelCreate(ChannelPayload),
    ChannelUpdate(ChannelPayload),
    ChannelDelete(ChannelPayload),
    ChannelPinsUpdate(ChannelPinsPayload),
    GuildCreate(GuildPayload),
    GuildUpdate(GuildPayload),
    GuildDelete(GuildPayload),
    GuildBanAdd(GuildBanPayload),
    GuildBanRemove(GuildBanPayload),
    GuildEmojisUpdate(GuildEmojisPayload),
    GuildStickersUpdate(GuildStickersPayload),
    GuildIntegrationsUpdate(GuildIntegrationsPayload),
    GuildMemberAdd(GuildMemberPayload),
    GuildMemberRemove(GuildMemberPayload),
    GuildMemberUpdate(GuildMemberPayload),
    GuildRoleCreate(GuildRolePayload),
    GuildRoleUpdate(GuildRolePayload),
    GuildRoleDelete(GuildRoleDeletePayload),
    GuildScheduledEventCreate(GuildScheduledEventPayload),
    GuildScheduledEventUpdate(GuildScheduledEventPayload),
    GuildScheduledEventDelete(GuildScheduledEventPayload),
    InviteCreate(InvitePayload),
    InviteDelete(InvitePayload),
    MessageCreate(MessagePayload),
    MessageUpdate(MessagePayload),
    MessageDelete(MessageDeletePayload),
    MessageDeleteBulk(MessageDeleteBulkPayload),
    MessageReactionAdd(MessageReactionPayload),
    MessageReactionRemove(MessageReactionPayload),
    MessageReactionRemoveAll(MessageReactionRemoveAllPayload),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiPayload),
    PresenceUpdate(PresencePayload),
    TypingStart(TypingStartPayload),
    UserUpdate(UserUpdatePayload),
    VoiceStateUpdate(VoiceStatePayload),
    VoiceServerUpdate(VoiceServerPayload),
    WebhooksUpdate(WebhooksPayload),
    InteractionCreate(InteractionPayload),
    Unknown(UnknownDispatchEvent),
}

impl DispatchEvent {
    pub fn kind(&self) -> &str {
        match self {
            Self::Ready(_) => "READY",
            Self::Resumed(_) => "RESUMED",
            Self::ChannelCreate(_) => "CHANNEL_CREATE",
            Self::ChannelUpdate(_) => "CHANNEL_UPDATE",
            Self::ChannelDelete(_) => "CHANNEL_DELETE",
            Self::ChannelPinsUpdate(_) => "CHANNEL_PINS_UPDATE",
            Self::GuildCreate(_) => "GUILD_CREATE",
            Self::GuildUpdate(_) => "GUILD_UPDATE",
            Self::GuildDelete(_) => "GUILD_DELETE",
            Self::GuildBanAdd(_) => "GUILD_BAN_ADD",
            Self::GuildBanRemove(_) => "GUILD_BAN_REMOVE",
            Self::GuildEmojisUpdate(_) => "GUILD_EMOJIS_UPDATE",
            Self::GuildStickersUpdate(_) => "GUILD_STICKERS_UPDATE",
            Self::GuildIntegrationsUpdate(_) => "GUILD_INTEGRATIONS_UPDATE",
            Self::GuildMemberAdd(_) => "GUILD_MEMBER_ADD",
            Self::GuildMemberRemove(_) => "GUILD_MEMBER_REMOVE",
            Self::GuildMemberUpdate(_) => "GUILD_MEMBER_UPDATE",
            Self::GuildRoleCreate(_) => "GUILD_ROLE_CREATE",
            Self::GuildRoleUpdate(_) => "GUILD_ROLE_UPDATE",
            Self::GuildRoleDelete(_) => "GUILD_ROLE_DELETE",
            Self::GuildScheduledEventCreate(_) => "GUILD_SCHEDULED_EVENT_CREATE",
            Self::GuildScheduledEventUpdate(_) => "GUILD_SCHEDULED_EVENT_UPDATE",
            Self::GuildScheduledEventDelete(_) => "GUILD_SCHEDULED_EVENT_DELETE",
            Self::InviteCreate(_) => "INVITE_CREATE",
            Self::InviteDelete(_) => "INVITE_DELETE",
            Self::MessageCreate(_) => "MESSAGE_CREATE",
            Self::MessageUpdate(_) => "MESSAGE_UPDATE",
            Self::MessageDelete(_) => "MESSAGE_DELETE",
            Self::MessageDeleteBulk(_) => "MESSAGE_DELETE_BULK",
            Self::MessageReactionAdd(_) => "MESSAGE_REACTION_ADD",
            Self::MessageReactionRemove(_) => "MESSAGE_REACTION_REMOVE",
            Self::MessageReactionRemoveAll(_) => "MESSAGE_REACTION_REMOVE_ALL",
            Self::MessageReactionRemoveEmoji(_) => "MESSAGE_REACTION_REMOVE_EMOJI",
            Self::PresenceUpdate(_) => "PRESENCE_UPDATE",
            Self::TypingStart(_) => "TYPING_START",
            Self::UserUpdate(_) => "USER_UPDATE",
            Self::VoiceStateUpdate(_) => "VOICE_STATE_UPDATE",
            Self::VoiceServerUpdate(_) => "VOICE_SERVER_UPDATE",
            Self::WebhooksUpdate(_) => "WEBHOOKS_UPDATE",
            Self::InteractionCreate(_) => "INTERACTION_CREATE",
            Self::Unknown(v) => v.event_type.as_str(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyPayload {
    pub session_id: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ResumedPayload {}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelPayload {
    pub id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelPinsPayload {
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub last_pin_timestamp: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildPayload {
    pub id: Snowflake,
    #[serde(default)]
    pub unavailable: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildBanPayload {
    pub guild_id: Snowflake,
    pub user: Value,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildEmojisPayload {
    pub guild_id: Snowflake,
    pub emojis: Vec<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildStickersPayload {
    pub guild_id: Snowflake,
    pub stickers: Vec<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildIntegrationsPayload {
    pub guild_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildMemberPayload {
    pub guild_id: Snowflake,
    #[serde(default)]
    pub user: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildRolePayload {
    pub guild_id: Snowflake,
    pub role: Value,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildRoleDeletePayload {
    pub guild_id: Snowflake,
    pub role_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildScheduledEventPayload {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvitePayload {
    pub code: String,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessagePayload {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeletePayload {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeleteBulkPayload {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageReactionPayload {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub emoji: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageReactionRemoveAllPayload {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageReactionRemoveEmojiPayload {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub emoji: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PresencePayload {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub user: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypingStartPayload {
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdatePayload {
    pub id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoiceStatePayload {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoiceServerPayload {
    pub token: String,
    pub guild_id: Snowflake,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhooksPayload {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InteractionPayload {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: i32,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct UnknownDispatchEvent {
    pub event_type: String,
    pub raw: Value,
}

pub fn decode_dispatch(event: &GatewayEvent) -> Result<Option<DispatchEnvelope>> {
    if event.op != 0 {
        return Ok(None);
    }

    let seq = event.s.unwrap_or(0);
    let kind = event.t.clone().unwrap_or_else(|| "UNKNOWN".to_owned());
    let payload = event.d.clone();

    let decoded = match kind.as_str() {
        "READY" => DispatchEvent::Ready(parse(payload, &kind)?),
        "RESUMED" => DispatchEvent::Resumed(parse(payload, &kind)?),
        "CHANNEL_CREATE" => DispatchEvent::ChannelCreate(parse(payload, &kind)?),
        "CHANNEL_UPDATE" => DispatchEvent::ChannelUpdate(parse(payload, &kind)?),
        "CHANNEL_DELETE" => DispatchEvent::ChannelDelete(parse(payload, &kind)?),
        "CHANNEL_PINS_UPDATE" => DispatchEvent::ChannelPinsUpdate(parse(payload, &kind)?),
        "GUILD_CREATE" => DispatchEvent::GuildCreate(parse(payload, &kind)?),
        "GUILD_UPDATE" => DispatchEvent::GuildUpdate(parse(payload, &kind)?),
        "GUILD_DELETE" => DispatchEvent::GuildDelete(parse(payload, &kind)?),
        "GUILD_BAN_ADD" => DispatchEvent::GuildBanAdd(parse(payload, &kind)?),
        "GUILD_BAN_REMOVE" => DispatchEvent::GuildBanRemove(parse(payload, &kind)?),
        "GUILD_EMOJIS_UPDATE" => DispatchEvent::GuildEmojisUpdate(parse(payload, &kind)?),
        "GUILD_STICKERS_UPDATE" => DispatchEvent::GuildStickersUpdate(parse(payload, &kind)?),
        "GUILD_INTEGRATIONS_UPDATE" => {
            DispatchEvent::GuildIntegrationsUpdate(parse(payload, &kind)?)
        }
        "GUILD_MEMBER_ADD" => DispatchEvent::GuildMemberAdd(parse(payload, &kind)?),
        "GUILD_MEMBER_REMOVE" => DispatchEvent::GuildMemberRemove(parse(payload, &kind)?),
        "GUILD_MEMBER_UPDATE" => DispatchEvent::GuildMemberUpdate(parse(payload, &kind)?),
        "GUILD_ROLE_CREATE" => DispatchEvent::GuildRoleCreate(parse(payload, &kind)?),
        "GUILD_ROLE_UPDATE" => DispatchEvent::GuildRoleUpdate(parse(payload, &kind)?),
        "GUILD_ROLE_DELETE" => DispatchEvent::GuildRoleDelete(parse(payload, &kind)?),
        "GUILD_SCHEDULED_EVENT_CREATE" => {
            DispatchEvent::GuildScheduledEventCreate(parse(payload, &kind)?)
        }
        "GUILD_SCHEDULED_EVENT_UPDATE" => {
            DispatchEvent::GuildScheduledEventUpdate(parse(payload, &kind)?)
        }
        "GUILD_SCHEDULED_EVENT_DELETE" => {
            DispatchEvent::GuildScheduledEventDelete(parse(payload, &kind)?)
        }
        "INVITE_CREATE" => DispatchEvent::InviteCreate(parse(payload, &kind)?),
        "INVITE_DELETE" => DispatchEvent::InviteDelete(parse(payload, &kind)?),
        "MESSAGE_CREATE" => DispatchEvent::MessageCreate(parse(payload, &kind)?),
        "MESSAGE_UPDATE" => DispatchEvent::MessageUpdate(parse(payload, &kind)?),
        "MESSAGE_DELETE" => DispatchEvent::MessageDelete(parse(payload, &kind)?),
        "MESSAGE_DELETE_BULK" => DispatchEvent::MessageDeleteBulk(parse(payload, &kind)?),
        "MESSAGE_REACTION_ADD" => DispatchEvent::MessageReactionAdd(parse(payload, &kind)?),
        "MESSAGE_REACTION_REMOVE" => DispatchEvent::MessageReactionRemove(parse(payload, &kind)?),
        "MESSAGE_REACTION_REMOVE_ALL" => {
            DispatchEvent::MessageReactionRemoveAll(parse(payload, &kind)?)
        }
        "MESSAGE_REACTION_REMOVE_EMOJI" => {
            DispatchEvent::MessageReactionRemoveEmoji(parse(payload, &kind)?)
        }
        "PRESENCE_UPDATE" => DispatchEvent::PresenceUpdate(parse(payload, &kind)?),
        "TYPING_START" => DispatchEvent::TypingStart(parse(payload, &kind)?),
        "USER_UPDATE" => DispatchEvent::UserUpdate(parse(payload, &kind)?),
        "VOICE_STATE_UPDATE" => DispatchEvent::VoiceStateUpdate(parse(payload, &kind)?),
        "VOICE_SERVER_UPDATE" => DispatchEvent::VoiceServerUpdate(parse(payload, &kind)?),
        "WEBHOOKS_UPDATE" => DispatchEvent::WebhooksUpdate(parse(payload, &kind)?),
        "INTERACTION_CREATE" => DispatchEvent::InteractionCreate(parse(payload, &kind)?),
        _ => DispatchEvent::Unknown(UnknownDispatchEvent {
            event_type: kind,
            raw: payload,
        }),
    };

    Ok(Some(DispatchEnvelope {
        seq,
        event: decoded,
    }))
}

fn parse<T: for<'de> Deserialize<'de>>(payload: Value, event_type: &str) -> Result<T> {
    serde_json::from_value(payload).map_err(|e| {
        ProtocolError::InvalidPayload(format!("failed to decode {event_type}: {e}")).into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_ready() {
        let event = GatewayEvent {
            op: 0,
            s: Some(1),
            t: Some("READY".to_owned()),
            d: serde_json::json!({"session_id":"abc","v":1}),
        };

        let decoded = decode_dispatch(&event).expect("decode").expect("dispatch");
        match decoded.event {
            DispatchEvent::Ready(v) => assert_eq!(v.session_id, "abc"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn decode_unknown() {
        let event = GatewayEvent {
            op: 0,
            s: Some(7),
            t: Some("SOMETHING_NEW".to_owned()),
            d: serde_json::json!({"x":1}),
        };

        let decoded = decode_dispatch(&event).expect("decode").expect("dispatch");
        match decoded.event {
            DispatchEvent::Unknown(v) => {
                assert_eq!(v.event_type, "SOMETHING_NEW");
                assert_eq!(v.raw, serde_json::json!({"x":1}));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn non_dispatch_none() {
        let event = GatewayEvent {
            op: 11,
            s: None,
            t: None,
            d: Value::Null,
        };
        let decoded = decode_dispatch(&event).expect("decode");
        assert!(decoded.is_none());
    }
}
