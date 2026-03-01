use serde::{Deserialize, Serialize};

use crate::Snowflake;

/// Role from `GET /guilds/{id}/roles`.
///
/// `permissions` is a bitfield as string (e.g. `"8933636165185"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRole {
    pub id: Snowflake,
    pub name: String,
    pub color: u32,
    pub position: i32,
    #[serde(default)]
    pub hoist_position: Option<i32>,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
    #[serde(default)]
    pub unicode_emoji: Option<String>,
}

/// Body for `POST /guilds/{id}/roles`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRoleBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist_position: Option<i32>,
}

/// Body for `PATCH /guilds/{id}/roles/{roleId}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRoleBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist_position: Option<i32>,
}
