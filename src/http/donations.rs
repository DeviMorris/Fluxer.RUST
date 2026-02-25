use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct DonationsApi {
    http: HttpClient,
}

impl DonationsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn manage(&self) -> Result<DonationManageResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/donations/manage")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), DonationManageResponse>(&ep, None)
            .await
    }

    pub async fn checkout(&self, body: &DonationActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/donations/checkout")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<DonationActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn request_link(&self, body: &DonationActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/donations/request-link")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<DonationActionRequest, Value>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationManageResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DonationActionRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
