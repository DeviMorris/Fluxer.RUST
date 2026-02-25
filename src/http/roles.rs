use super::members::MemberResponse;
use crate::error::Result;
use crate::flags::Permissions;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::tri::Patch;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct RolesApi {
    http: HttpClient,
}

impl RolesApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn create_role(
        &self,
        guild_id: Snowflake,
        body: &CreateRoleRequest,
    ) -> Result<RoleResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/roles")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<CreateRoleRequest, RoleResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_role(
        &self,
        guild_id: Snowflake,
        role_id: Snowflake,
        body: &UpdateRoleRequest,
    ) -> Result<RoleResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/roles/{role.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("role.id", &role_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<UpdateRoleRequest, RoleResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_role(&self, guild_id: Snowflake, role_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/guilds/{guild.id}/roles/{role.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("role.id", &role_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn modify_member_roles(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        body: &ModifyMemberRolesRequest,
    ) -> Result<MemberResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/members/{user.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<ModifyMemberRolesRequest, MemberResponse>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub name: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub permissions: Patch<Permissions>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub color: Patch<u32>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub hoist: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub mentionable: Patch<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyMemberRolesRequest {
    pub roles: Vec<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleResponse {
    pub id: Snowflake,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
