use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::union::PartialUser;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct MembersApi {
    http: HttpClient,
}

impl MembersApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<MemberResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/members/{user.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<(), MemberResponse>(&ep, None)
            .await
    }

    pub async fn list_members(
        &self,
        guild_id: Snowflake,
        query: &ListMembersQuery,
    ) -> Result<Vec<MemberResponse>> {
        let mut q = QueryValues::new();
        q.insert_opt("limit", query.limit);
        q.insert_opt("after", query.after);

        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/members")
            .compile(&q, &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<MemberResponse>>(&ep, None)
            .await
    }

    pub async fn kick_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/guilds/{guild.id}/members/{user.id}")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild.id", &guild_id.to_string()),
                    ("user.id", &user_id.to_string()),
                ],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn ban_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        body: &BanMemberRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Put, "/guilds/{guild.id}/bans/{user.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn unban_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/guilds/{guild.id}/bans/{user.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMembersQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<Snowflake>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BanMemberRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<PartialUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
