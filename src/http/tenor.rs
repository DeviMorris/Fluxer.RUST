use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct TenorApi {
    http: HttpClient,
}

impl TenorApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn featured(&self) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/tenor/featured")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn search(&self, query: &TenorQuery) -> Result<Value> {
        let mut q = QueryValues::new();
        q.insert_opt("q", query.q.as_ref());
        q.insert_opt("locale", query.locale.as_ref());
        q.insert_opt("limit", query.limit);
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/tenor/search").compile(&q, &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn suggest(&self, query: &TenorQuery) -> Result<Value> {
        let mut q = QueryValues::new();
        q.insert_opt("q", query.q.as_ref());
        q.insert_opt("locale", query.locale.as_ref());
        q.insert_opt("limit", query.limit);
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/tenor/suggest").compile(&q, &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn trending_gifs(&self) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/tenor/trending-gifs")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn register_share(&self, body: &TenorShareRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/tenor/register-share")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenorQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenorShareRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
