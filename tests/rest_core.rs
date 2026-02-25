use fluxer_rust::enums::ChannelType;
use fluxer_rust::http::{
    BanMemberRequest, ChannelsApi, CreateChannelRequest, GuildsApi, HttpClient, HttpClientConfig,
    MembersApi, MessagesApi, RetryPolicy, SendMessageRequest,
};
use fluxer_rust::id::Snowflake;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

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
    };
    HttpClient::new(cfg).expect("http client")
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
                "id": "100",
                "channel_id": "1",
                "content": "ok"
            }))
        }
    }
}

#[tokio::test]
async fn send_msg_retry() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    Mock::given(method("POST"))
        .and(path("/channels/1/messages"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(serde_json::json!({"content":"hello"})))
        .respond_with(Flaky429 { hits: hits.clone() })
        .mount(&server)
        .await;

    let api = MessagesApi::new(make_http(server.uri()));
    let message = api
        .send_message(
            Snowflake::new(1),
            &SendMessageRequest {
                content: Some("hello".to_owned()),
                nonce: None,
                flags: None,
                tts: None,
                embeds: None,
                attachments: None,
                allowed_mentions: None,
                message_reference: None,
                components: None,
            },
        )
        .await
        .expect("send");

    assert_eq!(message.id, Snowflake::new(100));
    assert_eq!(hits.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn get_channel_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/channels/55"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "55",
            "type": 0,
            "guild_id": "9",
            "name": "general",
            "permission_overwrites": []
        })))
        .mount(&server)
        .await;

    let api = ChannelsApi::new(make_http(server.uri()));
    let channel = api.get_channel(Snowflake::new(55)).await.expect("channel");
    assert_eq!(channel.id, Snowflake::new(55));
    assert_eq!(channel.kind, ChannelType::GuildText);
}

#[tokio::test]
async fn create_channel_ok() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/guilds/9/channels"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(serde_json::json!({
            "name":"voice",
            "type":2
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"77",
            "type":2,
            "guild_id":"9",
            "name":"voice",
            "permission_overwrites":[]
        })))
        .mount(&server)
        .await;

    let api = ChannelsApi::new(make_http(server.uri()));
    let channel = api
        .create_channel(
            Snowflake::new(9),
            &CreateChannelRequest {
                name: "voice".to_owned(),
                kind: ChannelType::GuildVoice,
                topic: None,
                bitrate: None,
                user_limit: None,
                parent_id: None,
                nsfw: None,
                position: None,
                permission_overwrites: None,
            },
        )
        .await
        .expect("create");
    assert_eq!(channel.id, Snowflake::new(77));
}

#[tokio::test]
async fn delete_channel_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/channels/77"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"77",
            "type":0,
            "permission_overwrites":[]
        })))
        .mount(&server)
        .await;

    let api = ChannelsApi::new(make_http(server.uri()));
    let channel = api
        .delete_channel(Snowflake::new(77))
        .await
        .expect("delete");
    assert_eq!(channel.id, Snowflake::new(77));
}

#[tokio::test]
async fn kick_member_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/guilds/9/members/42"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = MembersApi::new(make_http(server.uri()));
    api.kick_member(Snowflake::new(9), Snowflake::new(42))
        .await
        .expect("kick");
}

#[tokio::test]
async fn ban_member_ok() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/guilds/9/bans/42"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .and(body_json(
            serde_json::json!({"delete_message_seconds":3600}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let api = MembersApi::new(make_http(server.uri()));
    api.ban_member(
        Snowflake::new(9),
        Snowflake::new(42),
        &BanMemberRequest {
            delete_message_seconds: Some(3600),
        },
    )
    .await
    .expect("ban");
}

#[tokio::test]
async fn get_guild_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/guilds/9"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id":"9",
            "name":"guild",
            "owner_id":"1"
        })))
        .mount(&server)
        .await;

    let api = GuildsApi::new(make_http(server.uri()));
    let guild = api.get_guild(Snowflake::new(9)).await.expect("guild");
    assert_eq!(guild.id, Snowflake::new(9));
    assert_eq!(guild.name, "guild");
}

#[tokio::test]
async fn get_member_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/guilds/9/members/42"))
        .and(header("authorization", "Bot TEST_TOKEN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "user":{"id":"42","username":"u"},
            "roles":["5","6"],
            "nick":"nick"
        })))
        .mount(&server)
        .await;

    let api = MembersApi::new(make_http(server.uri()));
    let member = api
        .get_member(Snowflake::new(9), Snowflake::new(42))
        .await
        .expect("member");
    assert_eq!(member.roles.len(), 2);
    assert_eq!(member.user.expect("user").id, Snowflake::new(42));
}
