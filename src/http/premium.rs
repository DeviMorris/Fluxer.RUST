use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct PremiumApi {
    http: HttpClient,
}

impl PremiumApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_price_ids(&self) -> Result<PremiumPriceIdsResponse> {
        let ep =
            Endpoint::new(HttpMethod::Get, "/premium/price-ids").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), PremiumPriceIdsResponse>(&ep, None)
            .await
    }

    pub async fn cancel_subscription(&self, body: &PremiumActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/premium/cancel-subscription")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PremiumActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn customer_portal(&self, body: &PremiumActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/premium/customer-portal")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PremiumActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn reactivate_subscription(&self, body: &PremiumActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/premium/reactivate-subscription")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PremiumActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn operator_rejoin(&self, body: &PremiumActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/premium/operator/rejoin")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PremiumActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn visionary_rejoin(&self, body: &PremiumActionRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/premium/visionary/rejoin")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PremiumActionRequest, Value>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumPriceIdsResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumActionRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
