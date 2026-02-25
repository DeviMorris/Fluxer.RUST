use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct SearchApi {
    http: HttpClient,
}

impl SearchApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn search_messages(
        &self,
        body: &SearchMessagesRequest,
    ) -> Result<SearchMessagesResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/search/messages")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<SearchMessagesRequest, SearchMessagesResponse>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
