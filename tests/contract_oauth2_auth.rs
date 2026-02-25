use fluxer_rust::error::Error;
use fluxer_rust::http::{AuthApi, AuthRequest, HttpClient, HttpClientConfig, RetryPolicy};
use fluxer_rust::id::Snowflake;
use fluxer_rust::oauth2::{
    OAuth2Client, SudoVerificationSchema, TokenRequest,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Match, Mock, MockServer, Request, Respond, ResponseTemplate};

fn make_http(base_url: String) -> HttpClient {
    let cfg = HttpClientConfig {
        base_url,
        bot_token: Some("TEST_TOKEN".to_owned()),
        user_agent: "fluxer-rust-test".to_owned(),
        timeout: Duration::from_secs(2),
        retry: RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
        },
        allow_env_proxy: false,
    };
    HttpClient::new(cfg).expect("http client")
}

struct NoAuth;

impl Match for NoAuth {
    fn matches(&self, req: &Request) -> bool {
        !req.headers.contains_key("authorization")
    }
}

struct Flaky429 {
    hits: Arc<AtomicUsize>,
}

impl Respond for Flaky429 {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let prev = self.hits.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .insert_header("x-ratelimit-global", "true")
                .set_body_string("rate limited")
        } else {
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "sub":"1",
                "id":"1",
                "username":"u",
                "discriminator":"0001",
                "global_name":null,
                "avatar":null
            }))
        }
    }
}

#[tokio::test]
async fn oauth_me_bearer() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/oauth2/@me"))
        .and(header("authorization", "Bearer TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "application":{"id":"10","name":"app","bot_public":true,"bot_require_code_grant":false},
            "scopes":["identify"],
            "expires":"2026-01-01T00:00:00.000Z"
        })))
        .mount(&server)
        .await;

    let api = OAuth2Client::new(make_http(server.uri()));
    let me = api.me("TOKEN").await.expect("me");
    assert_eq!(me.application.id, Snowflake::new(10));
}

#[tokio::test]
async fn oauth_delete_sudo_body() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/oauth2/applications/10"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(serde_json::json!({"password":"pw"})))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = OAuth2Client::new(make_http(server.uri()));
    api.delete_application_with_sudo(
        Snowflake::new(10),
        &SudoVerificationSchema {
            password: Some("pw".to_owned()),
            ..Default::default()
        },
    )
    .await
    .expect("delete");
}

#[tokio::test]
async fn auth_login_no_bot() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .and(NoAuth)
        .and(body_json(serde_json::json!({"email":"a@b.c"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok":true})))
        .mount(&server)
        .await;

    let api = AuthApi::new(make_http(server.uri()));
    let mut body = AuthRequest::default();
    body.raw.insert("email".to_owned(), serde_json::json!("a@b.c"));
    let res = api.login(&body).await.expect("login");
    assert_eq!(res.get("ok"), Some(&serde_json::json!(true)));
}

#[tokio::test]
async fn token_400() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .respond_with(
            ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "code": 50035,
                "message":"Invalid Form Body"
            })),
        )
        .mount(&server)
        .await;

    let api = OAuth2Client::new(make_http(server.uri()));
    let err = api
        .token(&TokenRequest::RefreshToken {
            refresh_token: "bad".to_owned(),
            client_id: None,
            client_secret: None,
        })
        .await
        .expect_err("expected error");
    match err {
        Error::Api(api_err) => {
            assert_eq!(api_err.status, 400);
            assert_eq!(api_err.code, Some(50035));
        }
        _ => panic!("wrong error"),
    }
}

#[tokio::test]
async fn userinfo_retry_429() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/oauth2/userinfo"))
        .and(header("authorization", "Bearer TOKEN"))
        .respond_with(Flaky429 { hits: hits.clone() })
        .mount(&server)
        .await;

    let api = OAuth2Client::new(make_http(server.uri()));
    let info = api.userinfo("TOKEN").await.expect("userinfo");
    assert_eq!(info.id, Snowflake::new(1));
    assert_eq!(hits.load(Ordering::SeqCst), 2);
}
