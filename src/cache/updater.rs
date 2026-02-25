use super::model::{CachedChannel, CachedGuild, CachedMember, CachedRole};
use super::store::Caches;
use crate::flags::Permissions;
use crate::gateway::{DispatchEnvelope, DispatchEvent};
use crate::id::Snowflake;
use crate::union::PermissionOverwrite;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct CacheUpdater {
    caches: Arc<Caches>,
}

impl CacheUpdater {
    pub fn new(caches: Arc<Caches>) -> Self {
        Self { caches }
    }

    pub fn spawn(&self, mut rx: broadcast::Receiver<DispatchEnvelope>) -> JoinHandle<()> {
        let updater = self.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(ev) => updater.apply(&ev),
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        })
    }

    pub fn apply(&self, envelope: &DispatchEnvelope) {
        match &envelope.event {
            DispatchEvent::GuildCreate(v) | DispatchEvent::GuildUpdate(v) => {
                if let Some(guild) = parse_guild(v.id, &v.extra) {
                    self.caches.insert_guild(guild);
                }
            }
            DispatchEvent::GuildDelete(v) => {
                self.caches.remove_guild(v.id);
            }
            DispatchEvent::ChannelCreate(v) | DispatchEvent::ChannelUpdate(v) => {
                if let Some(channel) = parse_channel(v.id, v.guild_id, &v.extra) {
                    self.caches.insert_channel(channel);
                }
            }
            DispatchEvent::ChannelDelete(v) => {
                self.caches.remove_channel(v.id);
            }
            DispatchEvent::GuildMemberAdd(v) | DispatchEvent::GuildMemberUpdate(v) => {
                if let Some(member) =
                    parse_member(v.guild_id, v.user.as_ref(), &v.extra, &self.caches)
                {
                    self.caches.insert_member(member);
                }
            }
            DispatchEvent::GuildMemberRemove(v) => {
                if let Some(user_id) = member_user_id(v.user.as_ref(), &v.extra) {
                    self.caches.remove_member(v.guild_id, user_id);
                }
            }
            DispatchEvent::GuildRoleCreate(v) | DispatchEvent::GuildRoleUpdate(v) => {
                if let Some(role) = parse_role(v.guild_id, &v.role) {
                    self.caches.insert_role(role);
                }
            }
            DispatchEvent::GuildRoleDelete(v) => {
                self.caches.remove_role(v.guild_id, v.role_id);
            }
            _ => {}
        }
    }
}

fn parse_guild(id: Snowflake, extra: &Map<String, Value>) -> Option<CachedGuild> {
    #[derive(Deserialize)]
    struct GuildData {
        owner_id: Snowflake,
        #[serde(default)]
        name: String,
    }

    let data: GuildData = serde_json::from_value(Value::Object(extra.clone())).ok()?;
    Some(CachedGuild {
        id,
        owner_id: data.owner_id,
        name: data.name,
    })
}

fn parse_channel(
    id: Snowflake,
    guild_id: Option<Snowflake>,
    extra: &Map<String, Value>,
) -> Option<CachedChannel> {
    #[derive(Deserialize, Default)]
    struct ChannelData {
        #[serde(default)]
        permission_overwrites: Vec<PermissionOverwrite>,
    }

    let data: ChannelData = serde_json::from_value(Value::Object(extra.clone())).ok()?;
    Some(CachedChannel {
        id,
        guild_id,
        permission_overwrites: data.permission_overwrites,
    })
}

fn parse_member(
    guild_id: Snowflake,
    user: Option<&Value>,
    extra: &Map<String, Value>,
    caches: &Caches,
) -> Option<CachedMember> {
    #[derive(Deserialize, Default)]
    struct MemberData {
        #[serde(default)]
        roles: Vec<Snowflake>,
    }

    let user_id = member_user_id(user, extra)?;
    let parsed = serde_json::from_value::<MemberData>(Value::Object(extra.clone())).ok();
    let role_ids = if let Some(parsed) = parsed {
        if parsed.roles.is_empty() {
            caches
                .members
                .get(&guild_id, &user_id)
                .map(|m| m.role_ids)
                .unwrap_or_default()
        } else {
            parsed.roles
        }
    } else {
        caches
            .members
            .get(&guild_id, &user_id)
            .map(|m| m.role_ids)
            .unwrap_or_default()
    };

    Some(CachedMember {
        guild_id,
        user_id,
        role_ids,
    })
}

fn member_user_id(user: Option<&Value>, extra: &Map<String, Value>) -> Option<Snowflake> {
    #[derive(Deserialize)]
    struct UserId {
        id: Snowflake,
    }

    if let Some(user) = user {
        return serde_json::from_value::<UserId>(user.clone())
            .ok()
            .map(|v| v.id);
    }

    extra
        .get("user")
        .and_then(|v| {
            serde_json::from_value::<UserId>(v.clone())
                .ok()
                .map(|x| x.id)
        })
        .or_else(|| {
            extra
                .get("user_id")
                .and_then(|v| serde_json::from_value::<Snowflake>(v.clone()).ok())
        })
}

fn parse_role(guild_id: Snowflake, raw: &Value) -> Option<CachedRole> {
    #[derive(Deserialize, Default)]
    struct RoleData {
        id: Snowflake,
        #[serde(default)]
        permissions: Permissions,
        #[serde(default)]
        position: i32,
    }

    let data = serde_json::from_value::<RoleData>(raw.clone()).ok()?;
    Some(CachedRole {
        id: data.id,
        guild_id,
        permissions: data.permissions,
        position: data.position,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CachePolicy;
    use crate::gateway::{GatewayEvent, decode_dispatch};
    use serde_json::json;

    fn env(seq: u64, kind: &str, payload: Value) -> DispatchEnvelope {
        decode_dispatch(&GatewayEvent {
            op: 0,
            s: Some(seq),
            t: Some(kind.to_owned()),
            d: payload,
        })
        .expect("decode")
        .expect("dispatch")
    }

    #[test]
    fn guild_ops() {
        let caches = Arc::new(Caches::new(CachePolicy::default()));
        let updater = CacheUpdater::new(caches.clone());

        updater.apply(&env(
            1,
            "GUILD_CREATE",
            json!({"id":"1","owner_id":"2","name":"g"}),
        ));
        assert!(caches.guilds.get(&Snowflake::new(1)).is_some());

        updater.apply(&env(2, "GUILD_DELETE", json!({"id":"1"})));
        updater.apply(&env(3, "GUILD_DELETE", json!({"id":"1"})));
        assert!(caches.guilds.get(&Snowflake::new(1)).is_none());
    }

    #[test]
    fn channel_ops() {
        let caches = Arc::new(Caches::new(CachePolicy::default()));
        let updater = CacheUpdater::new(caches.clone());

        updater.apply(&env(
            1,
            "CHANNEL_CREATE",
            json!({
                "id":"10",
                "guild_id":"1",
                "permission_overwrites": [{"id":"1","type":0,"allow":"1024","deny":"0"}]
            }),
        ));
        assert!(caches.channels.get(&Snowflake::new(10)).is_some());

        updater.apply(&env(2, "CHANNEL_DELETE", json!({"id":"10","guild_id":"1"})));
        assert!(caches.channels.get(&Snowflake::new(10)).is_none());
    }

    #[test]
    fn member_ops() {
        let caches = Arc::new(Caches::new(CachePolicy::default()));
        let updater = CacheUpdater::new(caches.clone());

        updater.apply(&env(
            1,
            "GUILD_MEMBER_ADD",
            json!({"guild_id":"1","user":{"id":"11"},"roles":["2","3"]}),
        ));
        let member = caches
            .members
            .get(&Snowflake::new(1), &Snowflake::new(11))
            .expect("member");
        assert_eq!(member.role_ids.len(), 2);

        updater.apply(&env(
            2,
            "GUILD_MEMBER_REMOVE",
            json!({"guild_id":"1","user":{"id":"11"}}),
        ));
        updater.apply(&env(
            3,
            "GUILD_MEMBER_REMOVE",
            json!({"guild_id":"1","user":{"id":"11"}}),
        ));
        assert!(
            caches
                .members
                .get(&Snowflake::new(1), &Snowflake::new(11))
                .is_none()
        );
    }

    #[test]
    fn role_ops() {
        let caches = Arc::new(Caches::new(CachePolicy::default()));
        let updater = CacheUpdater::new(caches.clone());

        updater.apply(&env(
            1,
            "GUILD_ROLE_CREATE",
            json!({"guild_id":"1","role":{"id":"5","permissions":"1024","position":1}}),
        ));
        assert!(
            caches
                .roles
                .get(&Snowflake::new(1), &Snowflake::new(5))
                .is_some()
        );

        updater.apply(&env(
            2,
            "GUILD_ROLE_DELETE",
            json!({"guild_id":"1","role_id":"5"}),
        ));
        assert!(
            caches
                .roles
                .get(&Snowflake::new(1), &Snowflake::new(5))
                .is_none()
        );
    }
}
