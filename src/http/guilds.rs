use super::channels::ChannelResponse;
use super::roles::RoleResponse;
use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct GuildsApi {
    http: HttpClient,
}

impl GuildsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_guild(&self, guild_id: Snowflake) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http.request_json::<(), GuildResponse>(&ep, None).await
    }

    pub async fn list_guild_channels(&self, guild_id: Snowflake) -> Result<Vec<ChannelResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/channels")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<ChannelResponse>>(&ep, None)
            .await
    }

    pub async fn get_guild_roles(&self, guild_id: Snowflake) -> Result<Vec<RoleResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/roles")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<RoleResponse>>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildResponse {
    pub id: Snowflake,
    pub name: String,
    pub owner_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
