use serde::{Deserialize, Serialize};

use crate::Snowflake;

/// Partial user object returned by the API.
///
/// Not all fields are guaranteed. Fields like `banner` are only
/// available when fetching a user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub avatar_color: Option<u32>,
    #[serde(default)]
    pub flags: Option<u32>,
    #[serde(default)]
    pub public_flags: Option<u32>,
    #[serde(default)]
    pub bot: Option<bool>,
    #[serde(default)]
    pub system: Option<bool>,
    #[serde(default)]
    pub banner: Option<String>,
}

/// User profile sub-object from `GET /users/{id}/profile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUserProfile {
    #[serde(default)]
    pub pronouns: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<u32>,
    #[serde(default)]
    pub banner_color: Option<u32>,
    #[serde(default)]
    pub theme: Option<String>,
}

/// Connected account from profile response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConnectedAccount {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
}

/// Full profile response from `GET /users/{id}/profile`.
///
/// Pass `?guild_id=GUILD_ID` for server-specific profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiProfileResponse {
    #[serde(default)]
    pub user_profile: Option<ApiUserProfile>,
    #[serde(default)]
    pub mutual_guilds: Option<Vec<MutualGuild>>,
    #[serde(default)]
    pub mutual_guild_ids: Option<Vec<Snowflake>>,
    #[serde(default)]
    pub connected_accounts: Option<Vec<ApiConnectedAccount>>,
}

/// Mutual guild entry in a profile response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualGuild {
    pub id: Snowflake,
}

/// Guild member from `GET /guilds/{guild_id}/members/{user_id}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuildMember {
    pub user: ApiUser,
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<u32>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub deaf: Option<bool>,
    #[serde(default)]
    pub communication_disabled_until: Option<String>,
    #[serde(default)]
    pub profile_flags: Option<u32>,
    #[serde(default)]
    pub premium_since: Option<String>,
}
