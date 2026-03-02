use serde::{Deserialize, Serialize};

use crate::user::ApiUser;
use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmoji {
    pub id: Snowflake,
    pub name: String,
    pub animated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmojiWithUser {
    pub id: Snowflake,
    pub name: String,
    pub animated: bool,
    #[serde(default)]
    pub user: Option<ApiUser>,
}
