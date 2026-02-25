use crate::error::Result;
use crate::http::{AuthPolicy, Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct InteractionsApi {
    http: HttpClient,
}

impl InteractionsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn callback(
        &self,
        interaction_id: Snowflake,
        token: &str,
        body: &InteractionCallback,
    ) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/interactions/{id}/{token}/callback",
            auth: AuthPolicy::NoBot,
        }
        .compile(
            &QueryValues::new(),
            &[("id", &interaction_id.to_string()), ("token", token)],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionCallback {
    #[serde(rename = "type")]
    pub callback_type: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

