use crate::error::Result;
use crate::http::{AuthPolicy, Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct OAuth2Client {
    http: HttpClient,
}

impl OAuth2Client {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn me(&self, bearer_token: &str) -> Result<OAuth2MeResponse> {
        let ep = Endpoint {
            method: HttpMethod::Get,
            route: "/oauth2/@me",
            auth: AuthPolicy::NoBot,
        }
        .compile(&QueryValues::new(), &[])?;

        self.http
            .request_json_with_auth::<(), OAuth2MeResponse>(
                &ep,
                None,
                Some(&format!("Bearer {bearer_token}")),
            )
            .await
    }

    pub async fn userinfo(&self, bearer_token: &str) -> Result<OAuth2UserInfoResponse> {
        let ep = Endpoint {
            method: HttpMethod::Get,
            route: "/oauth2/userinfo",
            auth: AuthPolicy::NoBot,
        }
        .compile(&QueryValues::new(), &[])?;

        self.http
            .request_json_with_auth::<(), OAuth2UserInfoResponse>(
                &ep,
                None,
                Some(&format!("Bearer {bearer_token}")),
            )
            .await
    }

    pub async fn token(&self, req: &TokenRequest) -> Result<OAuth2TokenResponse> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/oauth2/token",
            auth: AuthPolicy::NoBot,
        }
        .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<TokenRequest, OAuth2TokenResponse>(&ep, Some(req))
            .await
    }

    pub async fn revoke(&self, req: &RevokeRequestForm) -> Result<()> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/oauth2/token/revoke",
            auth: AuthPolicy::NoBot,
        }
        .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(req)).await
    }

    pub async fn introspect(
        &self,
        req: &IntrospectRequestForm,
    ) -> Result<OAuth2IntrospectResponse> {
        let ep = Endpoint {
            method: HttpMethod::Post,
            route: "/oauth2/introspect",
            auth: AuthPolicy::NoBot,
        }
        .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<IntrospectRequestForm, OAuth2IntrospectResponse>(&ep, Some(req))
            .await
    }

    pub async fn app_public(&self, id: Snowflake) -> Result<ApplicationPublicResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/oauth2/applications/{id}/public")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<(), ApplicationPublicResponse>(&ep, None)
            .await
    }

    pub async fn get_authorizations(&self) -> Result<Vec<OAuth2AuthorizationResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/oauth2/@me/authorizations")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<OAuth2AuthorizationResponse>>(&ep, None)
            .await
    }

    pub async fn delete_authorization(&self, application_id: Snowflake) -> Result<()> {
        let ep =
            Endpoint::new(HttpMethod::Delete, "/oauth2/@me/authorizations/{applicationId}")
                .compile(
                    &QueryValues::new(),
                    &[("applicationId", &application_id.to_string())],
                )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_my_applications(&self) -> Result<Vec<OAuthApplicationResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/oauth2/applications/@me")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<OAuthApplicationResponse>>(&ep, None)
            .await
    }

    pub async fn authorize_consent(
        &self,
        body: &OAuthConsentRequest,
    ) -> Result<OAuthConsentResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/oauth2/authorize/consent")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<OAuthConsentRequest, OAuthConsentResponse>(&ep, Some(body))
            .await
    }

    pub async fn create_application(
        &self,
        body: &OAuthApplicationCreateRequest,
    ) -> Result<OAuthApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/oauth2/applications")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<OAuthApplicationCreateRequest, OAuthApplicationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn get_application(&self, id: Snowflake) -> Result<OAuthApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/oauth2/applications/{id}")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<(), OAuthApplicationResponse>(&ep, None)
            .await
    }

    pub async fn update_application(
        &self,
        id: Snowflake,
        body: &OAuthApplicationUpdateRequest,
    ) -> Result<OAuthApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/oauth2/applications/{id}")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<OAuthApplicationUpdateRequest, OAuthApplicationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn delete_application(&self, id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/oauth2/applications/{id}")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn update_bot(
        &self,
        id: Snowflake,
        body: &OAuthBotProfileUpdateRequest,
    ) -> Result<OAuthApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/oauth2/applications/{id}/bot")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<OAuthBotProfileUpdateRequest, OAuthApplicationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn reset_bot_token(&self, id: Snowflake) -> Result<OAuthBotTokenResetResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/oauth2/applications/{id}/bot/reset-token")
            .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<(), OAuthBotTokenResetResponse>(&ep, None)
            .await
    }

    pub async fn reset_client_secret(
        &self,
        id: Snowflake,
    ) -> Result<OAuthClientSecretResetResponse> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/oauth2/applications/{id}/client-secret/reset",
        )
        .compile(&QueryValues::new(), &[("id", &id.to_string())])?;
        self.http
            .request_json::<(), OAuthClientSecretResetResponse>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum TokenRequest {
    AuthorizationCode {
        code: String,
        redirect_uri: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_id: Option<Snowflake>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    },
    RefreshToken {
        refresh_token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_id: Option<Snowflake>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRequestForm {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectRequestForm {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2IntrospectResponse {
    pub active: bool,
    #[serde(default)]
    pub client_id: Option<Snowflake>,
    #[serde(default)]
    pub exp: Option<i32>,
    #[serde(default)]
    pub iat: Option<i32>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub sub: Option<Snowflake>,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2MeResponse {
    pub application: OAuth2MeApplication,
    pub scopes: Vec<String>,
    pub expires: String,
    #[serde(default)]
    pub user: Option<OAuth2MeUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2MeApplication {
    pub id: Snowflake,
    pub name: String,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2MeUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2UserInfoResponse {
    pub sub: Snowflake,
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPublicResponse {
    pub id: Snowflake,
    pub name: String,
    pub bot_public: bool,
    pub scopes: Vec<String>,
    pub redirect_uris: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub bot: Option<ApplicationBotResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationBotResponse {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2AuthorizationResponse {
    pub application: OAuth2AuthorizationApplication,
    #[serde(default)]
    pub scopes: Vec<String>,
    pub authorized_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2AuthorizationApplication {
    pub id: Snowflake,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub bot_public: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthApplicationCreateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_require_code_grant: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthApplicationUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_require_code_grant: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthApplicationResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_require_code_grant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<ApplicationBotResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConsentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    pub client_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConsentResponse {
    pub redirect_to: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthBotProfileUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_flags: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthBotTokenResetResponse {
    pub token: String,
    pub bot: ApplicationBotResponse,
}

pub type OAuthClientSecretResetResponse = OAuthApplicationResponse;
