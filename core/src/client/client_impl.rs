use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use dashmap::DashMap;
use serde_json::Value;
use tokio::sync::mpsc;

use fluxer_rest::{Rest, RestOptions};
use fluxer_types::gateway::{GatewayOpcode, GatewayPresenceUpdateSendData};
use fluxer_ws::{WebSocketManager, WebSocketManagerOptions, WsEvent};

use crate::structures::channel::Channel;
use crate::structures::guild::Guild;
use crate::structures::user::User;

type EventCallback = Box<
    dyn Fn(Value) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Client configuration.
#[derive(Debug, Clone, Default)]
pub struct ClientOptions {
    pub intents: u64,
    pub presence: Option<GatewayPresenceUpdateSendData>,
    pub rest: Option<RestOptions>,
    pub gateway_version: Option<String>,
}

/// Main Fluxer bot client.
///
/// Connects to the gateway, routes events, provides REST access and cached structures.
///
/// # Examples
/// ```rust,ignore
/// use fluxer_core::{Client, ClientOptions, Events};
///
/// let mut client = Client::new(ClientOptions::default());
///
/// client.on(Events::MESSAGE_CREATE, |data| Box::pin(async move {
///     println!("New message: {:?}", data);
/// }));
///
/// client.login("Bot my-token").await?;
/// ```
pub struct Client {
    pub rest: Rest,
    pub guilds: DashMap<String, Guild>,
    pub channels: DashMap<String, Channel>,
    pub users: DashMap<String, User>,
    options: ClientOptions,
    handlers: HashMap<String, Vec<EventCallback>>,
    ready: bool,
    ready_at: Option<std::time::Instant>,
    user: Option<User>,
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        let rest = Rest::new(options.rest.clone().unwrap_or_default());
        Self {
            rest,
            guilds: DashMap::new(),
            channels: DashMap::new(),
            users: DashMap::new(),
            options,
            handlers: HashMap::new(),
            ready: false,
            ready_at: None,
            user: None,
        }
    }

    /// Register an event handler.
    ///
    /// `event` is one of the constants from `Events` (e.g. `Events::MESSAGE_CREATE`).
    /// The callback receives raw `serde_json::Value` dispatch data.
    pub fn on<F, Fut>(&mut self, event: &str, callback: F)
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let wrapped: EventCallback = Box::new(move |data| Box::pin(callback(data)));
        self.handlers
            .entry(event.to_string())
            .or_default()
            .push(wrapped);
    }

    /// The authenticated bot user. `None` until `READY`.
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    /// Whether the client has received the `READY` event.
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Get or create a user from API data, caching in `self.users`.
    pub fn get_or_create_user(&self, data: &fluxer_types::user::ApiUser) -> User {
        if let Some(mut existing) = self.users.get_mut(&data.id) {
            existing.patch(data);
            return existing.clone();
        }
        let user = User::from_api(data);
        self.users.insert(user.id.clone(), user.clone());
        user
    }

    /// Connect to the Fluxer gateway and authenticate.
    ///
    /// This method blocks until the client is destroyed or the process exits.
    ///
    /// # Arguments
    /// * `token` - Bot token (e.g. `"Bot my-token"`)
    ///
    /// # Errors
    /// Returns [`Error::AlreadyLoggedIn`] if already connected.
    /// Returns [`Error::Rest`] if the gateway fetch fails.
    pub async fn login(&mut self, token: &str) -> crate::Result<()> {
        if self.ready {
            return Err(crate::Error::AlreadyLoggedIn);
        }

        self.rest.set_token(token).await;

        let (ws_tx, mut ws_rx) = mpsc::unbounded_channel::<WsEvent>();

        let ws_options = WebSocketManagerOptions {
            token: token.to_string(),
            intents: self.options.intents,
            presence: self.options.presence.clone(),
            shard_ids: None,
            shard_count: None,
            version: self.options.gateway_version.clone().unwrap_or("1".to_string()),
        };

        let mut manager = WebSocketManager::new(ws_options, self.rest.clone(), ws_tx);
        manager.connect().await.map_err(crate::Error::Rest)?;

        while let Some(event) = ws_rx.recv().await {
            match event {
                WsEvent::ShardReady { data, .. } => {
                    if let Some(user_data) = data.get("user") {
                        if let Ok(api_user) = serde_json::from_value::<fluxer_types::user::ApiUser>(
                            user_data.clone(),
                        ) {
                            self.user = Some(User::from_api(&api_user));
                            self.users.insert(api_user.id.clone(), User::from_api(&api_user));
                        }
                    }

                    if let Some(guilds_arr) = data.get("guilds").and_then(|v| v.as_array()) {
                        for guild_val in guilds_arr {
                            if let Ok(api_guild) = serde_json::from_value::<fluxer_types::guild::ApiGuild>(
                                guild_val.clone(),
                            ) {
                                let guild = Guild::from_api(&api_guild);
                                self.guilds.insert(guild.id.clone(), guild);

                                if let Some(channels_arr) = guild_val.get("channels").and_then(|v| v.as_array()) {
                                    for ch_val in channels_arr {
                                        if let Ok(api_ch) = serde_json::from_value::<fluxer_types::channel::ApiChannel>(
                                            ch_val.clone(),
                                        ) {
                                            let ch = Channel::from_api(&api_ch);
                                            self.channels.insert(ch.id.clone(), ch);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    self.ready = true;
                    self.ready_at = Some(std::time::Instant::now());
                    self.emit_event("READY", Value::Null).await;
                }
                WsEvent::Dispatch { payload, .. } => {
                    if payload.op == GatewayOpcode::Dispatch {
                        if let Some(event_name) = &payload.t {
                            let data = payload.d.clone().unwrap_or(Value::Null);
                            self.handle_dispatch(event_name, &data).await;
                            self.emit_event(event_name, data).await;
                        }
                    }
                }
                WsEvent::Error { error, shard_id: _ } => {
                    self.emit_event("ERROR", Value::String(error)).await;
                }
                WsEvent::Debug(msg) => {
                    self.emit_event("DEBUG", Value::String(msg)).await;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Handle specific dispatch events to update cache.
    async fn handle_dispatch(&self, event: &str, data: &Value) {
        match event {
            "MESSAGE_CREATE" => {
                if let Some(author) = data.get("author") {
                    if let Ok(api_user) = serde_json::from_value::<fluxer_types::user::ApiUser>(
                        author.clone(),
                    ) {
                        self.get_or_create_user(&api_user);
                    }
                }
            }
            "GUILD_CREATE" => {
                if let Ok(api_guild) = serde_json::from_value::<fluxer_types::guild::ApiGuild>(
                    data.clone(),
                ) {
                    let guild = Guild::from_api(&api_guild);
                    self.guilds.insert(guild.id.clone(), guild);

                    if let Some(channels_arr) = data.get("channels").and_then(|v| v.as_array()) {
                        for ch_val in channels_arr {
                            if let Ok(api_ch) = serde_json::from_value::<fluxer_types::channel::ApiChannel>(
                                ch_val.clone(),
                            ) {
                                let ch = Channel::from_api(&api_ch);
                                self.channels.insert(ch.id.clone(), ch);
                            }
                        }
                    }
                }
            }
            "GUILD_UPDATE" => {
                if let Ok(api_guild) = serde_json::from_value::<fluxer_types::guild::ApiGuild>(
                    data.clone(),
                ) {
                    let guild = Guild::from_api(&api_guild);
                    self.guilds.insert(guild.id.clone(), guild);
                }
            }
            "GUILD_DELETE" => {
                if let Some(id) = data.get("id").and_then(|v| v.as_str()) {
                    self.guilds.remove(id);
                }
            }
            "CHANNEL_CREATE" | "CHANNEL_UPDATE" => {
                if let Ok(api_ch) = serde_json::from_value::<fluxer_types::channel::ApiChannel>(
                    data.clone(),
                ) {
                    let ch = Channel::from_api(&api_ch);
                    self.channels.insert(ch.id.clone(), ch);
                }
            }
            "CHANNEL_DELETE" => {
                if let Some(id) = data.get("id").and_then(|v| v.as_str()) {
                    self.channels.remove(id);
                }
            }
            _ => {}
        }
    }

    async fn emit_event(&self, event: &str, data: Value) {
        if let Some(handlers) = self.handlers.get(event) {
            for handler in handlers {
                handler(data.clone()).await;
            }
        }
    }

    /// Disconnect from the gateway and clear cached data.
    pub fn destroy(&mut self) {
        self.ready = false;
        self.ready_at = None;
        self.user = None;
        self.guilds.clear();
        self.channels.clear();
        self.users.clear();
    }
}
