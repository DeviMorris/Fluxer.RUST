use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct ReportsApi {
    http: HttpClient,
}

impl ReportsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn report_message(&self, body: &ReportRequest) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/message")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportRequest, ReportResponse>(&ep, Some(body))
            .await
    }

    pub async fn report_user(&self, body: &ReportRequest) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/user")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportRequest, ReportResponse>(&ep, Some(body))
            .await
    }

    pub async fn report_guild(&self, body: &ReportRequest) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/guild")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportRequest, ReportResponse>(&ep, Some(body))
            .await
    }

    pub async fn report_dsa(&self, body: &ReportRequest) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/dsa")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportRequest, ReportResponse>(&ep, Some(body))
            .await
    }

    pub async fn report_dsa_email_send(
        &self,
        body: &ReportDsaEmailRequest,
    ) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/dsa/email/send")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportDsaEmailRequest, ReportResponse>(&ep, Some(body))
            .await
    }

    pub async fn report_dsa_email_verify(
        &self,
        body: &ReportDsaEmailRequest,
    ) -> Result<ReportResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/reports/dsa/email/verify")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<ReportDsaEmailRequest, ReportResponse>(&ep, Some(body))
            .await
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportDsaEmailRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
