use serde::{Deserialize, Serialize};

use crate::user::ApiUser;

/// Ban entry from `GET /guilds/{id}/bans`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiBan {
    pub user: ApiUser,
    pub reason: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}
