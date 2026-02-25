use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct WellKnownApi {
    http: HttpClient,
}

impl WellKnownApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn fluxer(&self) -> Result<WellKnownFluxerResponse> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/.well-known/fluxer")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), WellKnownFluxerResponse>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnownFluxerResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
