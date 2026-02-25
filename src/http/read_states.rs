use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct ReadStatesApi {
    http: HttpClient,
}

impl ReadStatesApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn ack_bulk(&self, body: &AckBulkRequest) -> Result<AckBulkResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/read-states/ack-bulk")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<AckBulkRequest, AckBulkResponse>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AckBulkRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckBulkResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
