use fluxer_rust::error::Error;
use fluxer_rust::http::{HttpClient, HttpClientConfig, InvitesApi, RetryPolicy};
use std::time::Duration;
use wiremock::matchers::{header, method};
use wiremock::{Match, Mock, MockServer, Request, ResponseTemplate};

fn make_http(base_url: String) -> HttpClient {
    let cfg = HttpClientConfig {
        base_url,
        bot_token: Some("TEST_TOKEN".to_owned()),
        user_agent: "fluxer-rust-test".to_owned(),
        timeout: Duration::from_secs(2),
        retry: RetryPolicy {
            max_retries: 1,
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

#[tokio::test]
async fn get_no_bot() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(NoAuth)
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code":"abc",
            "type":0,
            "temporary":false,
            "member_count":12
        })))
        .mount(&server)
        .await;

    let api = InvitesApi::new(make_http(server.uri()));
    let invite = api.get_invite("abc").await.expect("get");
    assert_eq!(invite.code, "abc");
    let reqs = server.received_requests().await.expect("requests");
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].method.as_str(), "GET");
    assert_eq!(reqs[0].url.path(), "/invites/abc");
}

#[tokio::test]
async fn accept_bot_auth() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code":"abc",
            "type":0,
            "temporary":false
        })))
        .mount(&server)
        .await;

    let api = InvitesApi::new(make_http(server.uri()));
    api.accept_invite("abc").await.expect("accept");
    let reqs = server.received_requests().await.expect("requests");
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].method.as_str(), "POST");
    assert_eq!(reqs[0].url.path(), "/invites/abc");
}

#[tokio::test]
async fn delete_204() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = InvitesApi::new(make_http(server.uri()));
    api.delete_invite("abc").await.expect("delete");
    let reqs = server.received_requests().await.expect("requests");
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].method.as_str(), "DELETE");
    assert_eq!(reqs[0].url.path(), "/invites/abc");
}

#[tokio::test]
async fn accept_403() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(serde_json::json!({"code":50013,"message":"Missing Permissions"})),
        )
        .mount(&server)
        .await;

    let api = InvitesApi::new(make_http(server.uri()));
    let err = api.accept_invite("abc").await.expect_err("expected error");
    let reqs = server.received_requests().await.expect("requests");
    assert_eq!(reqs.len(), 1);
    assert_eq!(reqs[0].url.path(), "/invites/abc");
    match err {
        Error::Api(api_err) => {
            assert_eq!(api_err.status, 403);
            assert_eq!(api_err.code, Some(50013));
        }
        _ => panic!("wrong error"),
    }
}
