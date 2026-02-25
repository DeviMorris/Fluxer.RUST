use fluxer_rust::error::Error;
use fluxer_rust::http::{
    CreatePrivateChannelRequest, HttpClient, HttpClientConfig, RetryPolicy, UserProfileUpdateRequest,
    UsersApi,
};
use fluxer_rust::id::Snowflake;
use std::time::Duration;
use wiremock::matchers::{body_json, header, method, path};
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
async fn get_me_bot_auth() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/@me"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"1",
            "username":"bot",
            "discriminator":"0001",
            "global_name":null,
            "avatar":null,
            "flags":0,
            "is_staff":false,
            "acls":[],
            "traits":[],
            "email":null,
            "phone":null,
            "bio":null,
            "pronouns":null,
            "accent_color":null,
            "banner":null,
            "banner_color":null,
            "mfa_enabled":false,
            "verified":true,
            "premium_type":null,
            "premium_since":null,
            "premium_until":null,
            "premium_will_cancel":false,
            "premium_billing_cycle":null,
            "premium_lifetime_sequence":null,
            "premium_badge_hidden":false,
            "premium_badge_masked":false,
            "premium_badge_timestamp_hidden":false,
            "premium_badge_sequence_hidden":false,
            "premium_purchase_disabled":false,
            "premium_enabled_override":false,
            "password_last_changed_at":null,
            "required_actions":null,
            "nsfw_allowed":true,
            "has_dismissed_premium_onboarding":false,
            "has_ever_purchased":false,
            "has_unread_gift_inventory":false,
            "unread_gift_inventory_count":0,
            "used_mobile_client":false,
            "pending_bulk_message_deletion":null
        })))
        .mount(&server)
        .await;

    let api = UsersApi::new(make_http(server.uri()));
    let me = api.get_me().await.expect("me");
    assert_eq!(me.id, Snowflake::new(1));
}

#[tokio::test]
async fn create_dm_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users/@me/channels"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(serde_json::json!({"recipient_id":"55"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"77",
            "type":1,
            "name":"dm",
            "permission_overwrites":[]
        })))
        .mount(&server)
        .await;

    let api = UsersApi::new(make_http(server.uri()));
    let dm = api
        .create_dm(&CreatePrivateChannelRequest {
            recipient_id: Some(Snowflake::new(55)),
            recipients: None,
        })
        .await
        .expect("dm");
    assert_eq!(dm.id, Snowflake::new(77));
}

#[tokio::test]
async fn pin_204() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/users/@me/channels/77/pin"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = UsersApi::new(make_http(server.uri()));
    api.pin_channel(Snowflake::new(77)).await.expect("pin");
}

#[tokio::test]
async fn get_user_no_bot() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/99"))
        .and(NoAuth)
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"99",
            "username":"u"
        })))
        .mount(&server)
        .await;

    let api = UsersApi::new(make_http(server.uri()));
    let user = api.get_user(Snowflake::new(99)).await.expect("user");
    assert_eq!(user.get("id"), Some(&serde_json::json!("99")));
}

#[tokio::test]
async fn update_me_404() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/users/@me"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "code": 10013,
                "message":"Unknown User"
            })),
        )
        .mount(&server)
        .await;

    let api = UsersApi::new(make_http(server.uri()));
    let err = api
        .update_me(&UserProfileUpdateRequest::default())
        .await
        .expect_err("expected error");
    match err {
        Error::Api(api_err) => {
            assert_eq!(api_err.status, 404);
            assert_eq!(api_err.code, Some(10013));
        }
        _ => panic!("wrong error"),
    }
}
