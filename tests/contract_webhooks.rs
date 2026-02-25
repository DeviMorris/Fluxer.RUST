use fluxer_rust::error::Error;
use fluxer_rust::http::{
    ExecuteWebhookRequest, HttpClient, HttpClientConfig, RetryPolicy, WebhookCreateRequest,
    WebhooksApi,
};
use fluxer_rust::id::Snowflake;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use wiremock::matchers::{body_json, header, method, path, query_param};
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
                "id": "9001",
                "channel_id": "42",
                "content": "ok"
            }))
        }
    }
}

#[tokio::test]
async fn execute_wait_query() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/webhooks/10/tk"))
        .and(query_param("wait", "true"))
        .and(query_param("thread_id", "77"))
        .and(NoAuth)
        .and(body_json(serde_json::json!({"content":"hello"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"101",
            "channel_id":"42",
            "content":"hello"
        })))
        .mount(&server)
        .await;

    let api = WebhooksApi::new(make_http(server.uri()));
    let msg = api
        .execute_webhook(
            Snowflake::new(10),
            "tk",
            &ExecuteWebhookRequest {
                content: Some("hello".to_owned()),
                ..Default::default()
            },
            Some(true),
            Some(Snowflake::new(77)),
        )
        .await
        .expect("execute")
        .expect("message");

    assert_eq!(msg.id, Snowflake::new(101));
}

#[tokio::test]
async fn create_bot_auth() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/channels/42/webhooks"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(serde_json::json!({"name":"hook"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"55",
            "guild_id":"9",
            "channel_id":"42",
            "name":"hook",
            "token":"abc",
            "user":{"id":"1","username":"u"}
        })))
        .mount(&server)
        .await;

    let api = WebhooksApi::new(make_http(server.uri()));
    let hook = api
        .create_channel_webhook(
            Snowflake::new(42),
            &WebhookCreateRequest {
                name: "hook".to_owned(),
                avatar: None,
            },
        )
        .await
        .expect("create");

    assert_eq!(hook.id, Snowflake::new(55));
}

#[tokio::test]
async fn delete_204() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/webhooks/99"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = WebhooksApi::new(make_http(server.uri()));
    api.delete_webhook(Snowflake::new(99)).await.expect("delete");
}

#[tokio::test]
async fn get_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/webhooks/404"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({"code": 10015, "message":"Unknown Webhook"})),
        )
        .mount(&server)
        .await;

    let api = WebhooksApi::new(make_http(server.uri()));
    let err = api
        .get_webhook(Snowflake::new(404))
        .await
        .expect_err("expected error");

    match err {
        Error::Api(api_err) => {
            assert_eq!(api_err.status, 404);
            assert_eq!(api_err.code, Some(10015));
        }
        _ => panic!("wrong error"),
    }
}

#[tokio::test]
async fn retry_429() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    Mock::given(method("POST"))
        .and(path("/webhooks/10/tk"))
        .and(query_param("wait", "true"))
        .respond_with(Flaky429 { hits: hits.clone() })
        .mount(&server)
        .await;

    let api = WebhooksApi::new(make_http(server.uri()));
    let msg = api
        .execute_webhook(
            Snowflake::new(10),
            "tk",
            &ExecuteWebhookRequest {
                content: Some("x".to_owned()),
                ..Default::default()
            },
            Some(true),
            None,
        )
        .await
        .expect("execute")
        .expect("message");

    assert_eq!(msg.id, Snowflake::new(9001));
    assert_eq!(hits.load(Ordering::SeqCst), 2);
}
