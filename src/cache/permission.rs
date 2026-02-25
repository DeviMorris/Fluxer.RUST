use super::model::{CachedChannel, CachedMember, CachedRole};
use crate::flags::Permissions;
use crate::id::Snowflake;
use crate::union::PermissionOverwrite;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PermissionContext {
    pub guild_id: Snowflake,
    pub owner_id: Snowflake,
    pub user_id: Snowflake,
    pub role_ids: Vec<Snowflake>,
}

pub fn guild_permissions(
    ctx: &PermissionContext,
    roles: &HashMap<Snowflake, CachedRole>,
) -> Permissions {
    if ctx.user_id == ctx.owner_id {
        return Permissions::all();
    }

    let mut perms = Permissions::empty();

    if let Some(everyone) = roles.get(&ctx.guild_id) {
        perms |= everyone.permissions;
    }

    for role_id in &ctx.role_ids {
        if let Some(role) = roles.get(role_id) {
            perms |= role.permissions;
        }
    }

    if perms.contains(Permissions::ADMINISTRATOR) {
        Permissions::all()
    } else {
        perms
    }
}

pub fn channel_permissions(
    base: Permissions,
    guild_id: Snowflake,
    member: &CachedMember,
    channel: &CachedChannel,
) -> Permissions {
    if base.contains(Permissions::ADMINISTRATOR) {
        return Permissions::all();
    }

    let mut perms = base;
    apply_everyone_overwrite(&mut perms, guild_id, &channel.permission_overwrites);
    apply_role_overwrites(&mut perms, &member.role_ids, &channel.permission_overwrites);
    apply_member_overwrite(&mut perms, member.user_id, &channel.permission_overwrites);
    perms
}

fn apply_everyone_overwrite(
    perms: &mut Permissions,
    guild_id: Snowflake,
    overwrites: &[PermissionOverwrite],
) {
    for ov in overwrites {
        if let PermissionOverwrite::Role(v) = ov {
            if v.id == guild_id {
                *perms &= !v.deny;
                *perms |= v.allow;
            }
        }
    }
}

fn apply_role_overwrites(
    perms: &mut Permissions,
    role_ids: &[Snowflake],
    overwrites: &[PermissionOverwrite],
) {
    let role_set: HashSet<Snowflake> = role_ids.iter().copied().collect();
    let mut allow = Permissions::empty();
    let mut deny = Permissions::empty();

    for ov in overwrites {
        if let PermissionOverwrite::Role(v) = ov {
            if role_set.contains(&v.id) {
                allow |= v.allow;
                deny |= v.deny;
            }
        }
    }

    *perms &= !deny;
    *perms |= allow;
}

fn apply_member_overwrite(
    perms: &mut Permissions,
    user_id: Snowflake,
    overwrites: &[PermissionOverwrite],
) {
    for ov in overwrites {
        if let PermissionOverwrite::Member(v) = ov {
            if v.id == user_id {
                *perms &= !v.deny;
                *perms |= v.allow;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Permissions;

    #[test]
    fn owner_all() {
        let id = Snowflake::new(1);
        let ctx = PermissionContext {
            guild_id: id,
            owner_id: id,
            user_id: id,
            role_ids: vec![],
        };
        let perms = guild_permissions(&ctx, &HashMap::new());
        assert!(perms.contains(Permissions::ADMINISTRATOR));
    }

    #[test]
    fn admin_all() {
        let gid = Snowflake::new(1);
        let rid = Snowflake::new(2);
        let uid = Snowflake::new(3);
        let mut roles = HashMap::new();
        roles.insert(
            gid,
            CachedRole {
                id: gid,
                guild_id: gid,
                permissions: Permissions::VIEW_CHANNEL,
                position: 0,
            },
        );
        roles.insert(
            rid,
            CachedRole {
                id: rid,
                guild_id: gid,
                permissions: Permissions::ADMINISTRATOR,
                position: 1,
            },
        );
        let ctx = PermissionContext {
            guild_id: gid,
            owner_id: Snowflake::new(99),
            user_id: uid,
            role_ids: vec![rid],
        };
        let perms = guild_permissions(&ctx, &roles);
        assert!(perms.contains(Permissions::ADMINISTRATOR));
    }
}
