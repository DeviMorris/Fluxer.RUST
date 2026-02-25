use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct GiftsApi {
    http: HttpClient,
}

impl GiftsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_code(&self, code: &str) -> Result<GiftCodeResponse> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/gifts/{code}")
            .compile(&QueryValues::new(), &[("code", code)])?;
        self.http
            .request_json::<(), GiftCodeResponse>(&ep, None)
            .await
    }

    pub async fn redeem_code(&self, code: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/gifts/{code}/redeem")
            .compile(&QueryValues::new(), &[("code", code)])?;
        self.http.request_unit::<()>(&ep, None).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCodeResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
