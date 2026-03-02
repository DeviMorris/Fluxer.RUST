![logo](assets/logo.png)

# Fluxer.RUST

[![Crates.io](https://img.shields.io/crates/v/fluxer-core)](https://crates.io/crates/fluxer-core)
[![Docs](https://img.shields.io/badge/docs-docs.devimorris.tech-blue)](https://docs.devimorris.tech)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Community](https://img.shields.io/badge/community-join-7289da)](https://fluxer.gg/2bkFSWcs)

Rust library for building bots on the [Fluxer](https://fluxer.app) platform.

Uses `tokio` for async runtime and `reqwest` for HTTP. No boilerplate - just write your handlers.

Written because other Fluxer libraries either didn't exist in Rust or were too low-level to be useful day-to-day. This one covers the full API surface with typed events, built-in rate limiting, and proper error types.

---

## Workspace

| Crate | What it does |
|-------|-------------|
| `fluxer-core` | Client, typed events, all high-level structures |
| `fluxer-rest` | HTTP client with rate limiting and retries |
| `fluxer-ws` | WebSocket gateway |
| `fluxer-types` | Raw API types and route definitions |
| `fluxer-builders` | Message and embed builders |
| `fluxer-util` | CDN URLs, permissions, emoji helpers |

---

## Quick Start

```toml
# Cargo.toml
[package]
name = "test"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxer-core     = "0.1"
fluxer-builders = "0.1"
fluxer-rest     = "0.1"
tokio              = { version = "1", features = ["rt-multi-thread", "macros"] }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing            = "0.1"
```

```rust
// Main.rs
use fluxer_core::client::{Client, ClientOptions};
use fluxer_core::client::typed_events::DispatchEvent;
use fluxer_builders::{EmbedBuilder, MessagePayload};
use fluxer_rest::Rest;

const TOKEN: &str = "TOKEN";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let options = ClientOptions {
        intents: 0,
        wait_for_guilds: true,
        ..Default::default()
    };

    let mut client = Client::new(options);
    let rest: Rest = client.rest.clone();

    client.on_typed(move |event| {
        let rest = rest.clone();
        Box::pin(async move {
            match event {
                DispatchEvent::Ready => {
                    tracing::info!("Bot is ready");
                }

                DispatchEvent::MessageCreate { message, .. } => {
                    if message.content.trim() == "!ping" {
                        let embed = EmbedBuilder::new()
                            .title("Pong!")
                            .color(0x5865F2)
                            .build();

                        let payload = MessagePayload::new().add_embed(embed).build();

                        if let Err(e) = message.send(&rest, &payload).await {
                            tracing::error!("send: {e}");
                        }
                    }
                }

                _ => {}
            }
        })
    });

    if let Err(e) = client.login(TOKEN).await {
        tracing::error!("login: {e:?}");
    }
}
```

---

## What's covered

- Async from the ground up (`tokio`)
- Typed gateway events - no manual JSON parsing
- Rate limiter built-in
- Message & embed builders
- Message and reaction collectors
- REST: messages, channels, roles, webhooks, emojis, stickers, invites, bans, members

---

## Community

[Fluxer.RUST Community](https://fluxer.gg/2bkFSWcs)

---

## License

MIT [LICENSE](LICENSE)
