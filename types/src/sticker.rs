use serde::{Deserialize, Serialize};

use crate::Snowflake;
use crate::user::ApiUser;

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
