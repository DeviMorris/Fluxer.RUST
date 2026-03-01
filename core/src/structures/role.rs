use fluxer_types::role::ApiRole;
use fluxer_types::Snowflake;
use fluxer_util::{parse_permissions, Permissions};

/// A role in a guild.
#[derive(Debug, Clone)]
pub struct Role {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub color: u32,
    pub position: i32,
    pub permissions_raw: String,
    pub hoist: bool,
    pub mentionable: bool,
    pub unicode_emoji: Option<String>,
    pub hoist_position: Option<i32>,
}

impl Role {
    pub fn from_api(data: &ApiRole, guild_id: &str) -> Self {
        Self {
            id: data.id.clone(),
            guild_id: guild_id.to_string(),
            name: data.name.clone(),
            color: data.color,
            position: data.position,
            permissions_raw: data.permissions.clone(),
            hoist: data.hoist,
            mentionable: data.mentionable,
            unicode_emoji: data.unicode_emoji.clone(),
            hoist_position: data.hoist_position,
        }
    }

    /// Parsed permission bitflags.
    ///
    /// Administrator implies all permissions.
    pub fn permissions(&self) -> Permissions {
        let perms = parse_permissions(&self.permissions_raw);
        if perms.contains(Permissions::ADMINISTRATOR) {
            Permissions::all()
        } else {
            perms
        }
    }

    /// Mention string (e.g. `<@&123456>`).
    pub fn mention(&self) -> String {
        format!("<@&{}>", self.id)
    }

    /// Update mutable fields from fresh API data.
    pub fn patch(&mut self, data: &ApiRole) {
        self.name.clone_from(&data.name);
        self.color = data.color;
        self.position = data.position;
        self.permissions_raw.clone_from(&data.permissions);
        self.hoist = data.hoist;
        self.mentionable = data.mentionable;
        self.unicode_emoji.clone_from(&data.unicode_emoji);
        self.hoist_position = data.hoist_position;
    }

    /// Edit this role.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn edit(
        &mut self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_types::role::UpdateRoleBody,
    ) -> crate::Result<()> {
        let data: ApiRole = rest
            .patch(
                &fluxer_types::Routes::guild_role(&self.guild_id, &self.id),
                Some(body),
            )
            .await?;
        self.patch(&data);
        Ok(())
    }

    /// Delete this role.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_role(&self.guild_id, &self.id))
            .await?;
        Ok(())
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@&{}>", self.id)
    }
}
