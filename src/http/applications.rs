use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct ApplicationsApi {
    http: HttpClient,
}

impl ApplicationsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_me(&self) -> Result<ApplicationMeResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/applications/@me")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), ApplicationMeResponse>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMeResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
