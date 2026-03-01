use serde::{Deserialize, Serialize};

use crate::user::ApiUser;
use crate::Snowflake;

/// Custom sticker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSticker {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub animated: bool,
}

/// Sticker with creator user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStickerWithUser {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub animated: bool,
    #[serde(default)]
    pub user: Option<ApiUser>,
}
