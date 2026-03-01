/// Gateway dispatch event names.
///
/// Use with `Client::on()` to register event handlers.
pub struct Events;

impl Events {
    pub const READY: &str = "READY";
    pub const RESUMED: &str = "RESUMED";
    pub const MESSAGE_CREATE: &str = "MESSAGE_CREATE";
    pub const MESSAGE_UPDATE: &str = "MESSAGE_UPDATE";
    pub const MESSAGE_DELETE: &str = "MESSAGE_DELETE";
    pub const MESSAGE_DELETE_BULK: &str = "MESSAGE_DELETE_BULK";
    pub const MESSAGE_REACTION_ADD: &str = "MESSAGE_REACTION_ADD";
    pub const MESSAGE_REACTION_REMOVE: &str = "MESSAGE_REACTION_REMOVE";
    pub const MESSAGE_REACTION_REMOVE_ALL: &str = "MESSAGE_REACTION_REMOVE_ALL";
    pub const MESSAGE_REACTION_REMOVE_EMOJI: &str = "MESSAGE_REACTION_REMOVE_EMOJI";
    pub const GUILD_CREATE: &str = "GUILD_CREATE";
    pub const GUILD_UPDATE: &str = "GUILD_UPDATE";
    pub const GUILD_DELETE: &str = "GUILD_DELETE";
    pub const GUILD_MEMBER_ADD: &str = "GUILD_MEMBER_ADD";
    pub const GUILD_MEMBER_UPDATE: &str = "GUILD_MEMBER_UPDATE";
    pub const GUILD_MEMBER_REMOVE: &str = "GUILD_MEMBER_REMOVE";
    pub const GUILD_MEMBERS_CHUNK: &str = "GUILD_MEMBERS_CHUNK";
    pub const GUILD_BAN_ADD: &str = "GUILD_BAN_ADD";
    pub const GUILD_BAN_REMOVE: &str = "GUILD_BAN_REMOVE";
    pub const GUILD_ROLE_CREATE: &str = "GUILD_ROLE_CREATE";
    pub const GUILD_ROLE_UPDATE: &str = "GUILD_ROLE_UPDATE";
    pub const GUILD_ROLE_DELETE: &str = "GUILD_ROLE_DELETE";
    pub const GUILD_EMOJIS_UPDATE: &str = "GUILD_EMOJIS_UPDATE";
    pub const GUILD_STICKERS_UPDATE: &str = "GUILD_STICKERS_UPDATE";
    pub const GUILD_INTEGRATIONS_UPDATE: &str = "GUILD_INTEGRATIONS_UPDATE";
    pub const CHANNEL_CREATE: &str = "CHANNEL_CREATE";
    pub const CHANNEL_UPDATE: &str = "CHANNEL_UPDATE";
    pub const CHANNEL_DELETE: &str = "CHANNEL_DELETE";
    pub const CHANNEL_PINS_UPDATE: &str = "CHANNEL_PINS_UPDATE";
    pub const INVITE_CREATE: &str = "INVITE_CREATE";
    pub const INVITE_DELETE: &str = "INVITE_DELETE";
    pub const TYPING_START: &str = "TYPING_START";
    pub const VOICE_STATE_UPDATE: &str = "VOICE_STATE_UPDATE";
    pub const VOICE_SERVER_UPDATE: &str = "VOICE_SERVER_UPDATE";
    pub const PRESENCE_UPDATE: &str = "PRESENCE_UPDATE";
    pub const WEBHOOKS_UPDATE: &str = "WEBHOOKS_UPDATE";
    pub const INTERACTION_CREATE: &str = "INTERACTION_CREATE";
    pub const USER_UPDATE: &str = "USER_UPDATE";
    pub const ERROR: &str = "ERROR";
    pub const DEBUG: &str = "DEBUG";
}
