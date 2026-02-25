use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct StripeApi {
    http: HttpClient,
}

impl StripeApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn checkout_gift(&self, body: &StripeRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/stripe/checkout/gift")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<StripeRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn checkout_subscription(&self, body: &StripeRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/stripe/checkout/subscription")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<StripeRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn webhook(&self, body: &StripeRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/stripe/webhook")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<StripeRequest, Value>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StripeRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
