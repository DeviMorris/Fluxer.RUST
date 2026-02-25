use crate::flags::Permissions;
use crate::id::Snowflake;
use crate::union::PermissionOverwrite;

#[derive(Debug, Clone)]
pub struct CachedGuild {
    pub id: Snowflake,
    pub owner_id: Snowflake,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CachedChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
}

#[derive(Debug, Clone)]
pub struct CachedRole {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub permissions: Permissions,
    pub position: i32,
}

#[derive(Debug, Clone)]
pub struct CachedMember {
    pub guild_id: Snowflake,
    pub user_id: Snowflake,
    pub role_ids: Vec<Snowflake>,
}

#[derive(Debug, Clone)]
pub struct CachedUser {
    pub id: Snowflake,
    pub bot: bool,
}
