use super::model::{CachedChannel, CachedGuild, CachedMember, CachedRole, CachedUser};
use super::permission::{PermissionContext, channel_permissions, guild_permissions};
use super::policy::{CacheKind, CachePolicy};
use crate::flags::Permissions;
use crate::id::Snowflake;
use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug)]
pub struct SingleCache<K, V>
where
    K: Eq + Hash,
{
    inner: DashMap<K, V>,
}

impl<K, V> Default for SingleCache<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }
}

impl<K, V> SingleCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).map(|v| v.clone())
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.remove(key).map(|(_, v)| v)
    }

    pub fn clear(&self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug)]
pub struct GroupedCache<G, K, V>
where
    G: Eq + Hash,
    K: Eq + Hash,
{
    groups: DashMap<G, Arc<DashMap<K, V>>>,
}

impl<G, K, V> Default for GroupedCache<G, K, V>
where
    G: Eq + Hash,
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            groups: DashMap::new(),
        }
    }
}

impl<G, K, V> GroupedCache<G, K, V>
where
    G: Eq + Hash + Clone,
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn insert(&self, group: G, key: K, value: V) {
        let bucket = self
            .groups
            .entry(group)
            .or_insert_with(|| Arc::new(DashMap::new()))
            .clone();
        bucket.insert(key, value);
    }

    pub fn get(&self, group: &G, key: &K) -> Option<V> {
        self.groups
            .get(group)
            .and_then(|b| b.get(key).map(|v| v.clone()))
    }

    pub fn remove(&self, group: &G, key: &K) -> Option<V> {
        self.groups
            .get(group)
            .and_then(|b| b.remove(key).map(|(_, v)| v))
    }

    pub fn group(&self, group: &G) -> Vec<V> {
        self.groups
            .get(group)
            .map(|b| b.iter().map(|v| v.value().clone()).collect())
            .unwrap_or_default()
    }

    pub fn group_map(&self, group: &G) -> std::collections::HashMap<K, V> {
        self.groups
            .get(group)
            .map(|b| {
                b.iter()
                    .map(|v| (v.key().clone(), v.value().clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn clear_group(&self, group: &G) {
        self.groups.remove(group);
    }

    pub fn clear(&self) {
        self.groups.clear();
    }
}

#[derive(Debug)]
pub struct Caches {
    policy: Arc<CachePolicy>,
    pub guilds: SingleCache<Snowflake, CachedGuild>,
    pub channels: SingleCache<Snowflake, CachedChannel>,
    pub roles: GroupedCache<Snowflake, Snowflake, CachedRole>,
    pub members: GroupedCache<Snowflake, Snowflake, CachedMember>,
    pub users: SingleCache<Snowflake, CachedUser>,
}

impl Caches {
    pub fn new(policy: CachePolicy) -> Self {
        Self {
            policy: Arc::new(policy),
            guilds: SingleCache::default(),
            channels: SingleCache::default(),
            roles: GroupedCache::default(),
            members: GroupedCache::default(),
            users: SingleCache::default(),
        }
    }

    pub fn policy(&self) -> &CachePolicy {
        &self.policy
    }

    pub fn insert_guild(&self, guild: CachedGuild) {
        if self.policy.should_cache(CacheKind::Guild) {
            self.guilds.insert(guild.id, guild);
        }
    }

    pub fn insert_channel(&self, channel: CachedChannel) {
        if self.policy.should_cache(CacheKind::Channel) {
            self.channels.insert(channel.id, channel);
        }
    }

    pub fn insert_role(&self, role: CachedRole) {
        if self.policy.should_cache(CacheKind::Role) {
            self.roles.insert(role.guild_id, role.id, role);
        }
    }

    pub fn insert_member(&self, member: CachedMember) {
        if self.policy.should_cache(CacheKind::Member) {
            self.members.insert(member.guild_id, member.user_id, member);
        }
    }

    pub fn insert_user(&self, user: CachedUser) {
        if self.policy.should_cache(CacheKind::User) {
            self.users.insert(user.id, user);
        }
    }

    pub fn member_permissions_in_guild(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<Permissions> {
        let guild = self.guilds.get(&guild_id)?;
        let member = self.members.get(&guild_id, &user_id)?;
        let ctx = PermissionContext {
            guild_id,
            owner_id: guild.owner_id,
            user_id,
            role_ids: member.role_ids.clone(),
        };
        let roles = self.roles.group_map(&guild_id);
        Some(guild_permissions(&ctx, &roles))
    }

    pub fn member_permissions_in_channel(
        &self,
        guild_id: Snowflake,
        channel_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<Permissions> {
        let member = self.members.get(&guild_id, &user_id)?;
        let channel = self.channels.get(&channel_id)?;
        let base = self.member_permissions_in_guild(guild_id, user_id)?;
        Some(channel_permissions(base, guild_id, &member, &channel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Permissions;

    #[test]
    fn single_ops() {
        let c = SingleCache::<u64, u64>::default();
        c.insert(1, 10);
        assert_eq!(c.get(&1), Some(10));
        assert_eq!(c.remove(&1), Some(10));
    }

    #[test]
    fn grouped_ops() {
        let c = GroupedCache::<u64, u64, u64>::default();
        c.insert(1, 10, 20);
        assert_eq!(c.get(&1, &10), Some(20));
        c.clear_group(&1);
        assert_eq!(c.get(&1, &10), None);
    }

    #[test]
    fn cache_policy() {
        let caches = Caches::new(CachePolicy {
            members: false,
            ..CachePolicy::default()
        });
        caches.insert_member(CachedMember {
            guild_id: Snowflake::new(1),
            user_id: Snowflake::new(2),
            role_ids: vec![],
        });
        assert_eq!(caches.members.group(&Snowflake::new(1)).len(), 0);
    }

    #[test]
    fn perms_in_channel() {
        let gid = Snowflake::new(1);
        let uid = Snowflake::new(2);
        let rid = Snowflake::new(3);
        let cid = Snowflake::new(4);

        let caches = Caches::new(CachePolicy::default());
        caches.insert_guild(CachedGuild {
            id: gid,
            owner_id: Snowflake::new(9),
            name: "g".to_owned(),
        });
        caches.insert_role(CachedRole {
            id: gid,
            guild_id: gid,
            permissions: Permissions::VIEW_CHANNEL,
            position: 0,
        });
        caches.insert_role(CachedRole {
            id: rid,
            guild_id: gid,
            permissions: Permissions::SEND_MESSAGES,
            position: 1,
        });
        caches.insert_member(CachedMember {
            guild_id: gid,
            user_id: uid,
            role_ids: vec![rid],
        });
        caches.insert_channel(CachedChannel {
            id: cid,
            guild_id: Some(gid),
            permission_overwrites: vec![],
        });

        let perms = caches
            .member_permissions_in_channel(gid, cid, uid)
            .expect("perms");
        assert!(perms.contains(Permissions::VIEW_CHANNEL));
        assert!(perms.contains(Permissions::SEND_MESSAGES));
    }
}
