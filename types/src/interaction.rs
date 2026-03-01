use serde::{Deserialize, Serialize};

use crate::user::{ApiGuildMember, ApiUser};
use crate::Snowflake;

/// Application command option value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandOptionValue {
    String(String),
    Number(f64),
    Bool(bool),
}

/// Application command option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOption {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: u8,
    #[serde(default)]
    pub value: Option<CommandOptionValue>,
}

/// Command interaction data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandData {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default, rename = "type")]
    pub kind: Option<u8>,
    #[serde(default)]
    pub options: Option<Vec<CommandOption>>,
}

/// Application command interaction from the gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiApplicationCommandInteraction {
    pub id: String,
    pub application_id: String,
    #[serde(rename = "type")]
    pub kind: u8,
    pub token: String,
    #[serde(default)]
    pub data: Option<CommandData>,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    #[serde(default)]
    pub member: Option<InteractionMember>,
    #[serde(default)]
    pub user: Option<ApiUser>,
}

/// Guild member with optional guild_id in interaction context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionMember {
    #[serde(flatten)]
    pub member: ApiGuildMember,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}
