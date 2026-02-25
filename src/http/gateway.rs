use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GatewayApi {
    http: HttpClient,
}

impl GatewayApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_gateway_bot(&self) -> Result<GatewayBotResponse> {
        let ep =
            Endpoint::new_no_bot_auth(HttpMethod::Get, "/gateway/bot").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), GatewayBotResponse>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayBotResponse {
    pub url: String,
    pub shards: i64,
    pub session_start_limit: GatewaySessionStartLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewaySessionStartLimit {
    pub total: i64,
    pub remaining: i64,
    pub reset_after: i64,
    pub max_concurrency: i64,
}
