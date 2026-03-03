use crate::structures::channel::Channel;
use crate::structures::guild::Guild;
use crate::structures::guild_ban::GuildBan;
use crate::structures::guild_member::GuildMember;
use crate::structures::invite::Invite;
use crate::structures::message::{Message, PartialMessage};
use crate::structures::message_reaction::MessageReaction;
use crate::structures::role::Role;
use crate::structures::user::User;

use fluxer_types::Snowflake;

#[derive(Debug, Clone)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum DispatchEvent {
    Ready,

    MessageCreate {
        message: Message,
        member: Option<GuildMember>,
    },

    MessageUpdate {
        message: Message,
    },

    MessageDelete {
        message: PartialMessage,
    },

    MessageDeleteBulk {
        ids: Vec<Snowflake>,
        channel_id: Snowflake,
        guild_id: Option<Snowflake>,
    },

    MessageReactionAdd {
        reaction: MessageReaction,
    },

    MessageReactionRemove {
        reaction: MessageReaction,
    },

    MessageReactionRemoveAll {
        message_id: Snowflake,
        channel_id: Snowflake,
        guild_id: Option<Snowflake>,
    },

    MessageReactionRemoveEmoji {
        message_id: Snowflake,
        channel_id: Snowflake,
        guild_id: Option<Snowflake>,
        emoji_name: String,
        emoji_id: Option<Snowflake>,
    },

    GuildCreate {
        guild: Guild,
    },

    GuildUpdate {
        guild: Guild,
    },

    GuildDelete {
        guild_id: Snowflake,
        unavailable: bool,
    },

    GuildMemberAdd {
        member: GuildMember,
    },

    GuildMemberUpdate {
        guild_id: Snowflake,
        user: User,
        nick: Option<String>,
        roles: Vec<Snowflake>,
    },

    GuildMemberRemove {
        guild_id: Snowflake,
        user: User,
    },

    GuildBanAdd {
        ban: GuildBan,
    },

    GuildBanRemove {
        guild_id: Snowflake,
        user: User,
    },

    GuildRoleCreate {
        guild_id: Snowflake,
        role: Role,
    },

    GuildRoleUpdate {
        guild_id: Snowflake,
        role: Role,
    },

    GuildRoleDelete {
        guild_id: Snowflake,
        role_id: Snowflake,
    },

    ChannelCreate {
        channel: Channel,
    },

    ChannelUpdate {
        channel: Channel,
    },

    ChannelDelete {
        channel: Channel,
    },

    InviteCreate {
        invite: Invite,
    },

    InviteDelete {
        code: String,
        channel_id: Snowflake,
        guild_id: Option<Snowflake>,
    },

    UserUpdate {
        user: User,
    },

    TypingStart {
        channel_id: Snowflake,
        user_id: Snowflake,
        guild_id: Option<Snowflake>,
        timestamp: u64,
    },

    VoiceStateUpdate {
        data: fluxer_types::gateway::GatewayVoiceStateUpdateData,
    },

    VoiceServerUpdate {
        data: fluxer_types::gateway::GatewayVoiceServerUpdateData,
    },

    PresenceUpdate {
        data: fluxer_types::gateway::GatewayPresenceUpdateData,
    },

    GuildEmojisUpdate {
        guild_id: Snowflake,
        emoji_ids: Vec<Snowflake>,
    },

    InteractionCreate {
        data: serde_json::Value,
    },

    Debug {
        message: String,
    },

    Error {
        message: String,
    },

    Raw {
        event_name: String,
        data: serde_json::Value,
    },
}
