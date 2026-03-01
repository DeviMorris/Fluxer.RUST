use bitflags::bitflags;

bitflags! {
    /// Permission flags aligned with the Fluxer API.
    ///
    /// Administrator (bit 3) implies all permissions.
    /// Guild owner bypasses role computation entirely.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Permissions: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS          = 1 << 1;
        const BAN_MEMBERS           = 1 << 2;
        const ADMINISTRATOR         = 1 << 3;
        const MANAGE_CHANNELS       = 1 << 4;
        const MANAGE_GUILD          = 1 << 5;
        const ADD_REACTIONS         = 1 << 6;
        const VIEW_AUDIT_LOG        = 1 << 7;
        const PRIORITY_SPEAKER      = 1 << 8;
        const STREAM                = 1 << 9;
        const VIEW_CHANNEL          = 1 << 10;
        const SEND_MESSAGES         = 1 << 11;
        const SEND_TTS_MESSAGES     = 1 << 12;
        const MANAGE_MESSAGES       = 1 << 13;
        const EMBED_LINKS           = 1 << 14;
        const ATTACH_FILES          = 1 << 15;
        const READ_MESSAGE_HISTORY  = 1 << 16;
        const MENTION_EVERYONE      = 1 << 17;
        const USE_EXTERNAL_EMOJIS   = 1 << 18;
        const CONNECT               = 1 << 20;
        const SPEAK                 = 1 << 21;
        const MUTE_MEMBERS          = 1 << 22;
        const DEAFEN_MEMBERS        = 1 << 23;
        const MOVE_MEMBERS          = 1 << 24;
        const USE_VAD               = 1 << 25;
        const CHANGE_NICKNAME       = 1 << 26;
        const MANAGE_NICKNAMES      = 1 << 27;
        const MANAGE_ROLES          = 1 << 28;
        const MANAGE_WEBHOOKS       = 1 << 29;
        const MANAGE_EXPRESSIONS    = 1 << 30;
        const USE_EXTERNAL_STICKERS = 2 << 37;
        const MODERATE_MEMBERS      = 2 << 40;
        const CREATE_EXPRESSIONS    = 2 << 43;
        const PIN_MESSAGES          = 2 << 51;
        const BYPASS_SLOWMODE       = 2 << 52;
        const UPDATE_RTC_REGION     = 2 << 53;
    }
}

/// All permissions OR'd together.
pub const ALL_PERMISSIONS: Permissions = Permissions::all();

/// Parse a permissions bitfield string (e.g. `"2048"`) into `Permissions`.
pub fn parse_permissions(s: &str) -> Permissions {
    let bits: u64 = s.parse().unwrap_or(0);
    Permissions::from_bits_truncate(bits)
}

/// Serialize `Permissions` back to a bitfield string for the API.
pub fn permissions_to_string(p: Permissions) -> String {
    p.bits().to_string()
}
