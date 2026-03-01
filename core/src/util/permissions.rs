use fluxer_types::channel::{ApiChannelOverwrite, OverwriteType};
use fluxer_util::Permissions;

/// Compute effective permissions for a member in a channel.
///
/// Guild owner always receives all permissions.
/// Administrator bit implies all permissions after overwrite computation.
///
/// # Arguments
/// * `base_permissions` - Combined permissions from all member roles
/// * `overwrites` - Channel permission overwrites
/// * `member_roles` - Role IDs the member has
/// * `member_id` - Member user ID (for member-specific overwrites)
/// * `is_owner` - Whether the member is the guild owner
pub fn compute_permissions(
    base_permissions: Permissions,
    overwrites: &[ApiChannelOverwrite],
    member_roles: &[String],
    member_id: &str,
    is_owner: bool,
) -> Permissions {
    if is_owner {
        return Permissions::all();
    }

    let mut perms = base_permissions;

    for overwrite in overwrites {
        let applies = match overwrite.kind {
            OverwriteType::Role => member_roles.iter().any(|r| r == &overwrite.id),
            OverwriteType::Member => overwrite.id == member_id,
        };
        if !applies {
            continue;
        }
        let allow = fluxer_util::parse_permissions(&overwrite.allow);
        let deny = fluxer_util::parse_permissions(&overwrite.deny);
        perms = (perms & !deny) | allow;
    }

    if perms.contains(Permissions::ADMINISTRATOR) {
        Permissions::all()
    } else {
        perms
    }
}

/// Check if a permission bitfield has a specific permission.
///
/// Administrator implies all permissions.
pub fn has_permission(bitfield: Permissions, permission: Permissions) -> bool {
    if bitfield.contains(Permissions::ADMINISTRATOR) {
        return true;
    }
    bitfield.contains(permission)
}
