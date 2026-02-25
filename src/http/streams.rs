use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct StreamsApi {
    http: HttpClient,
}

impl StreamsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_preview(&self, stream_key: &str) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/streams/{streamKey}/preview")
            .compile(&QueryValues::new(), &[("streamKey", stream_key)])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn create_preview(
        &self,
        stream_key: &str,
        body: &StreamPreviewRequest,
    ) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/streams/{streamKey}/preview")
            .compile(&QueryValues::new(), &[("streamKey", stream_key)])?;
        self.http
            .request_json::<StreamPreviewRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn update_stream(
        &self,
        stream_key: &str,
        body: &StreamUpdateRequest,
    ) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Patch, "/streams/{streamKey}/stream")
            .compile(&QueryValues::new(), &[("streamKey", stream_key)])?;
        self.http
            .request_json::<StreamUpdateRequest, Value>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamPreviewRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamUpdateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
