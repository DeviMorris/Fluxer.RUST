#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheKind {
    Guild,
    Channel,
    Role,
    Member,
    User,
}

#[derive(Debug, Clone)]
pub struct CachePolicy {
    pub auto_update: bool,
    pub guilds: bool,
    pub channels: bool,
    pub roles: bool,
    pub members: bool,
    pub users: bool,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self {
            auto_update: true,
            guilds: true,
            channels: true,
            roles: true,
            members: true,
            users: true,
        }
    }
}

impl CachePolicy {
    pub fn should_cache(&self, kind: CacheKind) -> bool {
        match kind {
            CacheKind::Guild => self.guilds,
            CacheKind::Channel => self.channels,
            CacheKind::Role => self.roles,
            CacheKind::Member => self.members,
            CacheKind::User => self.users,
        }
    }
}
