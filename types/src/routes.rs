/// Route builder helpers for the Fluxer REST API.
///
/// All routes are relative to `/v1`.
pub struct Routes;

impl Routes {
    pub fn channel(id: &str) -> String {
        format!("/channels/{id}")
    }

    pub fn channel_messages(id: &str) -> String {
        format!("/channels/{id}/messages")
    }

    pub fn channel_message(channel_id: &str, message_id: &str) -> String {
        format!("/channels/{channel_id}/messages/{message_id}")
    }

    pub fn channel_message_reactions(channel_id: &str, message_id: &str) -> String {
        format!("/channels/{channel_id}/messages/{message_id}/reactions")
    }

    pub fn channel_message_reaction(channel_id: &str, message_id: &str, emoji: &str) -> String {
        let encoded = urlencoding_encode(emoji);
        format!("/channels/{channel_id}/messages/{message_id}/reactions/{encoded}")
    }

    pub fn channel_pins(id: &str) -> String {
        format!("/channels/{id}/messages/pins")
    }

    pub fn channel_pin(channel_id: &str, message_id: &str) -> String {
        format!("/channels/{channel_id}/messages/pins/{message_id}")
    }

    pub fn channel_pin_message(channel_id: &str, message_id: &str) -> String {
        format!("/channels/{channel_id}/pins/{message_id}")
    }

    pub fn channel_bulk_delete(id: &str) -> String {
        format!("/channels/{id}/messages/bulk-delete")
    }

    pub fn channel_webhooks(id: &str) -> String {
        format!("/channels/{id}/webhooks")
    }

    pub fn channel_typing(id: &str) -> String {
        format!("/channels/{id}/typing")
    }

    pub fn channel_invites(id: &str) -> String {
        format!("/channels/{id}/invites")
    }

    pub fn channel_permission(channel_id: &str, overwrite_id: &str) -> String {
        format!("/channels/{channel_id}/permissions/{overwrite_id}")
    }

    pub fn channel_recipient(channel_id: &str, user_id: &str) -> String {
        format!("/channels/{channel_id}/recipients/{user_id}")
    }

    pub fn channel_message_attachment(
        channel_id: &str,
        message_id: &str,
        attachment_id: &str,
    ) -> String {
        format!("/channels/{channel_id}/messages/{message_id}/attachments/{attachment_id}")
    }

    pub fn guilds() -> &'static str {
        "/guilds"
    }

    pub fn guild(id: &str) -> String {
        format!("/guilds/{id}")
    }

    pub fn guild_delete(guild_id: &str) -> String {
        format!("/guilds/{guild_id}/delete")
    }

    pub fn guild_vanity_url(guild_id: &str) -> String {
        format!("/guilds/{guild_id}/vanity-url")
    }

    pub fn guild_channels(id: &str) -> String {
        format!("/guilds/{id}/channels")
    }

    pub fn guild_members(id: &str) -> String {
        format!("/guilds/{id}/members")
    }

    pub fn guild_member(guild_id: &str, user_id: &str) -> String {
        format!("/guilds/{guild_id}/members/{user_id}")
    }

    pub fn guild_member_role(guild_id: &str, user_id: &str, role_id: &str) -> String {
        format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}")
    }

    pub fn guild_roles(id: &str) -> String {
        format!("/guilds/{id}/roles")
    }

    pub fn guild_role(guild_id: &str, role_id: &str) -> String {
        format!("/guilds/{guild_id}/roles/{role_id}")
    }

    pub fn guild_bans(id: &str) -> String {
        format!("/guilds/{id}/bans")
    }

    pub fn guild_ban(guild_id: &str, user_id: &str) -> String {
        format!("/guilds/{guild_id}/bans/{user_id}")
    }

    pub fn guild_invites(id: &str) -> String {
        format!("/guilds/{id}/invites")
    }

    pub fn invite(code: &str) -> String {
        let encoded = urlencoding_encode(code);
        format!("/invites/{encoded}")
    }

    pub fn guild_audit_logs(id: &str) -> String {
        format!("/guilds/{id}/audit-logs")
    }

    pub fn guild_emojis(id: &str) -> String {
        format!("/guilds/{id}/emojis")
    }

    pub fn guild_emoji(guild_id: &str, emoji_id: &str) -> String {
        format!("/guilds/{guild_id}/emojis/{emoji_id}")
    }

    pub fn guild_stickers(id: &str) -> String {
        format!("/guilds/{id}/stickers")
    }

    pub fn guild_sticker(guild_id: &str, sticker_id: &str) -> String {
        format!("/guilds/{guild_id}/stickers/{sticker_id}")
    }

    pub fn guild_webhooks(id: &str) -> String {
        format!("/guilds/{id}/webhooks")
    }

    pub fn webhook(id: &str) -> String {
        format!("/webhooks/{id}")
    }

    pub fn webhook_execute(id: &str, token: &str) -> String {
        format!("/webhooks/{id}/{token}")
    }

    pub fn user(id: &str) -> String {
        format!("/users/{id}")
    }

    pub fn current_user() -> &'static str {
        "/users/@me"
    }

    pub fn current_user_guilds() -> &'static str {
        "/users/@me/guilds"
    }

    pub fn leave_guild(guild_id: &str) -> String {
        format!("/users/@me/guilds/{guild_id}")
    }

    pub fn user_me_channels() -> &'static str {
        "/users/@me/channels"
    }

    pub fn user_profile(id: &str, guild_id: Option<&str>) -> String {
        match guild_id {
            Some(gid) => format!("/users/{id}/profile?guild_id={gid}"),
            None => format!("/users/{id}/profile"),
        }
    }

    pub fn instance() -> &'static str {
        "/instance"
    }

    pub fn gateway_bot() -> &'static str {
        "/gateway/bot"
    }

    pub fn stream_preview(stream_key: &str) -> String {
        let encoded = urlencoding_encode(stream_key);
        format!("/streams/{encoded}/preview")
    }

    pub fn application_commands(application_id: &str) -> String {
        format!("/applications/{application_id}/commands")
    }

    pub fn application_command(application_id: &str, command_id: &str) -> String {
        format!("/applications/{application_id}/commands/{command_id}")
    }

    pub fn interaction_callback(interaction_id: &str, interaction_token: &str) -> String {
        format!("/interactions/{interaction_id}/{interaction_token}/callback")
    }
}

fn urlencoding_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    result
}
