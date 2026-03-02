
use serde_json::Value;

use crate::structures::channel::Channel;
use crate::structures::guild::Guild;
use crate::structures::guild_ban::GuildBan;
use crate::structures::guild_member::GuildMember;
use crate::structures::invite::Invite;
use crate::structures::message::{Message, PartialMessage};
use crate::structures::message_reaction::MessageReaction;
use crate::structures::role::Role;
use crate::structures::user::User;

use super::typed_events::DispatchEvent;

pub(crate) fn parse_dispatch(event_name: &str, data: &Value) -> DispatchEvent {
    match event_name {
        "MESSAGE_CREATE" => parse_message_create(data),
        "MESSAGE_UPDATE" => parse_message_update(data),
        "MESSAGE_DELETE" => parse_message_delete(data),
        "MESSAGE_DELETE_BULK" => parse_message_delete_bulk(data),
        "MESSAGE_REACTION_ADD" => parse_reaction_add(data),
        "MESSAGE_REACTION_REMOVE" => parse_reaction_remove(data),
        "MESSAGE_REACTION_REMOVE_ALL" => parse_reaction_remove_all(data),
        "MESSAGE_REACTION_REMOVE_EMOJI" => parse_reaction_remove_emoji(data),
        "GUILD_CREATE" => parse_guild_create(data),
        "GUILD_UPDATE" => parse_guild_update(data),
        "GUILD_DELETE" => parse_guild_delete(data),
        "GUILD_MEMBER_ADD" => parse_guild_member_add(data),
        "GUILD_MEMBER_UPDATE" => parse_guild_member_update(data),
        "GUILD_MEMBER_REMOVE" => parse_guild_member_remove(data),
        "GUILD_BAN_ADD" => parse_guild_ban_add(data),
        "GUILD_BAN_REMOVE" => parse_guild_ban_remove(data),
        "GUILD_ROLE_CREATE" => parse_guild_role_create(data),
        "GUILD_ROLE_UPDATE" => parse_guild_role_update(data),
        "GUILD_ROLE_DELETE" => parse_guild_role_delete(data),
        "GUILD_EMOJIS_UPDATE" => parse_guild_emojis_update(data),
        "CHANNEL_CREATE" => parse_channel_create(data),
        "CHANNEL_UPDATE" => parse_channel_update(data),
        "CHANNEL_DELETE" => parse_channel_delete(data),
        "INVITE_CREATE" => parse_invite_create(data),
        "INVITE_DELETE" => parse_invite_delete(data),
        "USER_UPDATE" => parse_user_update(data),
        "TYPING_START" => parse_typing_start(data),
        "VOICE_STATE_UPDATE" => parse_voice_state_update(data),
        "VOICE_SERVER_UPDATE" => parse_voice_server_update(data),
        "PRESENCE_UPDATE" => parse_presence_update(data),
        "INTERACTION_CREATE" => DispatchEvent::InteractionCreate {
            data: data.clone(),
        },
        _ => DispatchEvent::Raw {
            event_name: event_name.to_string(),
            data: data.clone(),
        },
    }
}


fn parse_message_create(data: &Value) -> DispatchEvent {
    let Some(message) = Message::from_value(data) else {
        return raw("MESSAGE_CREATE", data);
    };

    let member = parse_embedded_member(data);

    DispatchEvent::MessageCreate { message, member }
}

fn parse_message_update(data: &Value) -> DispatchEvent {
    match Message::from_value(data) {
        Some(message) => DispatchEvent::MessageUpdate { message },
        None => raw("MESSAGE_UPDATE", data),
    }
}

fn parse_message_delete(data: &Value) -> DispatchEvent {
    match PartialMessage::from_value(data) {
        Some(message) => DispatchEvent::MessageDelete { message },
        None => raw("MESSAGE_DELETE", data),
    }
}

fn parse_message_delete_bulk(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayMessageDeleteBulkData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::MessageDeleteBulk {
            ids: d.ids,
            channel_id: d.channel_id,
            guild_id: d.guild_id,
        },
        Err(_) => raw("MESSAGE_DELETE_BULK", data),
    }
}


fn parse_reaction_add(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayReactionAddData>(data.clone()) {
        Ok(d) => DispatchEvent::MessageReactionAdd {
            reaction: MessageReaction::from_gateway(&d),
        },
        Err(_) => raw("MESSAGE_REACTION_ADD", data),
    }
}

fn parse_reaction_remove(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayReactionRemoveData>(data.clone())
    {
        Ok(d) => {
            let reaction = MessageReaction {
                message_id: d.message_id,
                channel_id: d.channel_id,
                guild_id: d.guild_id,
                user_id: d.user_id,
                emoji_id: d.emoji.id,
                emoji_name: d.emoji.name,
                emoji_animated: d.emoji.animated.unwrap_or(false),
            };
            DispatchEvent::MessageReactionRemove { reaction }
        }
        Err(_) => raw("MESSAGE_REACTION_REMOVE", data),
    }
}

fn parse_reaction_remove_all(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayReactionRemoveAllData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::MessageReactionRemoveAll {
            message_id: d.message_id,
            channel_id: d.channel_id,
            guild_id: d.guild_id,
        },
        Err(_) => raw("MESSAGE_REACTION_REMOVE_ALL", data),
    }
}

fn parse_reaction_remove_emoji(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayReactionRemoveEmojiData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::MessageReactionRemoveEmoji {
            message_id: d.message_id,
            channel_id: d.channel_id,
            guild_id: d.guild_id,
            emoji_name: d.emoji.name,
            emoji_id: d.emoji.id,
        },
        Err(_) => raw("MESSAGE_REACTION_REMOVE_EMOJI", data),
    }
}


fn parse_guild_create(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::guild::ApiGuild>(data.clone()) {
        Ok(api_guild) => {
            let guild = Guild::from_api(&api_guild);
            DispatchEvent::GuildCreate { guild }
        }
        Err(_) => raw("GUILD_CREATE", data),
    }
}

fn parse_guild_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::guild::ApiGuild>(data.clone()) {
        Ok(api_guild) => {
            let guild = Guild::from_api(&api_guild);
            DispatchEvent::GuildUpdate { guild }
        }
        Err(_) => raw("GUILD_UPDATE", data),
    }
}

fn parse_guild_delete(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildDeleteData>(data.clone()) {
        Ok(d) => DispatchEvent::GuildDelete {
            guild_id: d.id,
            unavailable: d.unavailable.unwrap_or(false),
        },
        Err(_) => raw("GUILD_DELETE", data),
    }
}


fn parse_guild_member_add(data: &Value) -> DispatchEvent {
    let guild_id = str_field(data, "guild_id").unwrap_or_default();
    match serde_json::from_value::<fluxer_types::user::ApiGuildMember>(data.clone()) {
        Ok(api_m) => {
            let member = GuildMember::from_api(&api_m, &guild_id);
            DispatchEvent::GuildMemberAdd { member }
        }
        Err(_) => raw("GUILD_MEMBER_ADD", data),
    }
}

fn parse_guild_member_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildMemberUpdateData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::GuildMemberUpdate {
            guild_id: d.guild_id,
            user: User::from_api(&d.user),
            nick: d.nick,
            roles: d.roles,
        },
        Err(_) => raw("GUILD_MEMBER_UPDATE", data),
    }
}

fn parse_guild_member_remove(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildMemberRemoveData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::GuildMemberRemove {
            guild_id: d.guild_id,
            user: User::from_api(&d.user),
        },
        Err(_) => raw("GUILD_MEMBER_REMOVE", data),
    }
}


fn parse_guild_ban_add(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildBanAddData>(data.clone()) {
        Ok(d) => {
            let ban = GuildBan {
                guild_id: d.guild_id,
                user: User::from_api(&d.user),
                reason: d.reason,
                expires_at: None,
            };
            DispatchEvent::GuildBanAdd { ban }
        }
        Err(_) => raw("GUILD_BAN_ADD", data),
    }
}

fn parse_guild_ban_remove(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildBanRemoveData>(data.clone())
    {
        Ok(d) => DispatchEvent::GuildBanRemove {
            guild_id: d.guild_id,
            user: User::from_api(&d.user),
        },
        Err(_) => raw("GUILD_BAN_REMOVE", data),
    }
}


fn parse_guild_role_create(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildRoleData>(data.clone()) {
        Ok(d) => {
            let role = Role::from_api(&d.role, &d.guild_id);
            DispatchEvent::GuildRoleCreate {
                guild_id: d.guild_id,
                role,
            }
        }
        Err(_) => raw("GUILD_ROLE_CREATE", data),
    }
}

fn parse_guild_role_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildRoleData>(data.clone()) {
        Ok(d) => {
            let role = Role::from_api(&d.role, &d.guild_id);
            DispatchEvent::GuildRoleUpdate {
                guild_id: d.guild_id,
                role,
            }
        }
        Err(_) => raw("GUILD_ROLE_UPDATE", data),
    }
}

fn parse_guild_role_delete(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayGuildRoleDeleteData>(data.clone())
    {
        Ok(d) => DispatchEvent::GuildRoleDelete {
            guild_id: d.guild_id,
            role_id: d.role_id,
        },
        Err(_) => raw("GUILD_ROLE_DELETE", data),
    }
}


fn parse_guild_emojis_update(data: &Value) -> DispatchEvent {
    let guild_id = str_field(data, "guild_id").unwrap_or_default();
    let emoji_ids = data
        .get("emojis")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.get("id").and_then(|v| v.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    DispatchEvent::GuildEmojisUpdate {
        guild_id,
        emoji_ids,
    }
}


fn parse_channel_create(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::channel::ApiChannel>(data.clone()) {
        Ok(api_ch) => DispatchEvent::ChannelCreate {
            channel: Channel::from_api(&api_ch),
        },
        Err(_) => raw("CHANNEL_CREATE", data),
    }
}

fn parse_channel_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::channel::ApiChannel>(data.clone()) {
        Ok(api_ch) => DispatchEvent::ChannelUpdate {
            channel: Channel::from_api(&api_ch),
        },
        Err(_) => raw("CHANNEL_UPDATE", data),
    }
}

fn parse_channel_delete(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::channel::ApiChannel>(data.clone()) {
        Ok(api_ch) => DispatchEvent::ChannelDelete {
            channel: Channel::from_api(&api_ch),
        },
        Err(_) => raw("CHANNEL_DELETE", data),
    }
}


fn parse_invite_create(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::invite::ApiInvite>(data.clone()) {
        Ok(api_inv) => DispatchEvent::InviteCreate {
            invite: Invite::from_api(&api_inv),
        },
        Err(_) => raw("INVITE_CREATE", data),
    }
}

fn parse_invite_delete(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayInviteDeleteData>(data.clone()) {
        Ok(d) => DispatchEvent::InviteDelete {
            code: d.code,
            channel_id: d.channel_id,
            guild_id: d.guild_id,
        },
        Err(_) => raw("INVITE_DELETE", data),
    }
}


fn parse_user_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::user::ApiUser>(data.clone()) {
        Ok(api_user) => DispatchEvent::UserUpdate {
            user: User::from_api(&api_user),
        },
        Err(_) => raw("USER_UPDATE", data),
    }
}


fn parse_typing_start(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayTypingStartData>(data.clone()) {
        Ok(d) => DispatchEvent::TypingStart {
            channel_id: d.channel_id,
            user_id: d.user_id,
            guild_id: d.guild_id,
            timestamp: d.timestamp,
        },
        Err(_) => raw("TYPING_START", data),
    }
}


fn parse_voice_state_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayVoiceStateUpdateData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::VoiceStateUpdate { data: d },
        Err(_) => raw("VOICE_STATE_UPDATE", data),
    }
}

fn parse_voice_server_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayVoiceServerUpdateData>(
        data.clone(),
    ) {
        Ok(d) => DispatchEvent::VoiceServerUpdate { data: d },
        Err(_) => raw("VOICE_SERVER_UPDATE", data),
    }
}


fn parse_presence_update(data: &Value) -> DispatchEvent {
    match serde_json::from_value::<fluxer_types::gateway::GatewayPresenceUpdateData>(data.clone())
    {
        Ok(d) => DispatchEvent::PresenceUpdate { data: d },
        Err(_) => raw("PRESENCE_UPDATE", data),
    }
}


fn parse_embedded_member(data: &Value) -> Option<GuildMember> {
    let member_val = data.get("member")?;
    let guild_id = data.get("guild_id").and_then(|v| v.as_str())?;
    let author = data.get("author")?;

    let mut merged = member_val.clone();
    let obj = merged.as_object_mut()?;
    obj.insert("user".to_string(), author.clone());

    let api_member =
        serde_json::from_value::<fluxer_types::user::ApiGuildMember>(merged).ok()?;
    Some(GuildMember::from_api(&api_member, guild_id))
}

fn str_field(data: &Value, field: &str) -> Option<String> {
    data.get(field)
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn raw(event_name: &str, data: &Value) -> DispatchEvent {
    DispatchEvent::Raw {
        event_name: event_name.to_string(),
        data: data.clone(),
    }
}
