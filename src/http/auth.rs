use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct AuthApi {
    http: HttpClient,
}

impl AuthApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn login(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/login", body).await
    }

    pub async fn logout(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/logout", body).await
    }

    pub async fn register(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/register", body).await
    }

    pub async fn forgot(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/forgot", body).await
    }

    pub async fn reset(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/reset", body).await
    }

    pub async fn verify(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/verify", body).await
    }

    pub async fn resend_verify(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/verify/resend", body).await
    }

    pub async fn login_mfa_totp(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/login/mfa/totp", body).await
    }

    pub async fn login_mfa_sms(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/login/mfa/sms", body).await
    }

    pub async fn login_mfa_sms_send(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/login/mfa/sms/send", body).await
    }

    pub async fn login_mfa_webauthn(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/login/mfa/webauthn", body).await
    }

    pub async fn login_mfa_webauthn_options(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/login/mfa/webauthn/authentication-options", body)
            .await
    }

    pub async fn webauthn_authenticate(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/webauthn/authenticate", body).await
    }

    pub async fn webauthn_authentication_options(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/webauthn/authentication-options", body)
            .await
    }

    pub async fn authorize_ip(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/authorize-ip", body).await
    }

    pub async fn ip_authorization_poll(&self) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/auth/ip-authorization/poll")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn ip_authorization_resend(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/ip-authorization/resend", body).await
    }

    pub async fn sessions(&self) -> Result<Vec<AuthSessionResponse>> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/auth/sessions")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<AuthSessionResponse>>(&ep, None)
            .await
    }

    pub async fn sessions_logout(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/sessions/logout", body).await
    }

    pub async fn sso_start(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/sso/start", body).await
    }

    pub async fn sso_complete(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/sso/complete", body).await
    }

    pub async fn sso_status(&self) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/auth/sso/status")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn handoff_initiate(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/handoff/initiate", body).await
    }

    pub async fn handoff_complete(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/handoff/complete", body).await
    }

    pub async fn handoff_status(&self, code: &str) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/auth/handoff/{code}/status")
            .compile(&QueryValues::new(), &[("code", code)])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn handoff_delete(&self, code: &str) -> Result<()> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Delete, "/auth/handoff/{code}")
            .compile(&QueryValues::new(), &[("code", code)])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn username_suggestions(&self, body: &AuthRequest) -> Result<Value> {
        self.post_json("/auth/username-suggestions", body).await
    }

    pub async fn email_revert(&self, body: &AuthRequest) -> Result<()> {
        self.post_unit("/auth/email-revert", body).await
    }

    async fn post_json(&self, route: &'static str, body: &AuthRequest) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Post, route)
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AuthRequest, Value>(&ep, Some(body)).await
    }

    async fn post_unit(&self, route: &'static str, body: &AuthRequest) -> Result<()> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Post, route)
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit::<AuthRequest>(&ep, Some(body)).await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionResponse {
    pub id_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_info: Option<AuthSessionClientInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approx_last_used_at: Option<String>,
    pub current: bool,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionClientInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<AuthSessionLocation>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
