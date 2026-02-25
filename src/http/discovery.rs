use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct DiscoveryApi {
    http: HttpClient,
}

impl DiscoveryApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn list_categories(&self) -> Result<Vec<DiscoveryCategoryResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/discovery/categories")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<DiscoveryCategoryResponse>>(&ep, None)
            .await
    }

    pub async fn list_guilds(&self) -> Result<Vec<DiscoveryGuildResponse>> {
        let ep =
            Endpoint::new(HttpMethod::Get, "/discovery/guilds").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<DiscoveryGuildResponse>>(&ep, None)
            .await
    }

    pub async fn join_guild(&self, guild_id: Snowflake) -> Result<DiscoveryGuildResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/discovery/guilds/{guild.id}/join")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), DiscoveryGuildResponse>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryCategoryResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryGuildResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
