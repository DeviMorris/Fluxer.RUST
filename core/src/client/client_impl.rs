use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use dashmap::DashMap;
use serde_json::Value;
use tokio::sync::{RwLock, mpsc};
use tracing::warn;

use fluxer_rest::{Rest, RestOptions};
use fluxer_types::gateway::{GatewayOpcode, GatewayPresenceUpdateSendData};
use fluxer_types::message::ApiMessage;
use fluxer_ws::{WebSocketManager, WebSocketManagerOptions, WsEvent};

use crate::collectors::message_collector::{MessageCollector, MessageCollectorOptions};
use crate::collectors::reaction_collector::{
    CollectedReaction, ReactionCollector, ReactionCollectorOptions,
};
use crate::structures::channel::Channel;
use crate::structures::client_user::ClientUser;
use crate::structures::guild::Guild;
use crate::structures::guild_member::GuildMember;
use crate::structures::user::User;

use super::event_parser;
use super::typed_events::DispatchEvent;
use fluxer_voice::{FluxerVoiceConnection, VoiceError, VoiceManager};

type EventCallback = Box<dyn Fn(Value) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

type TypedEventCallback =
    Box<dyn Fn(DispatchEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Debug, Clone, Default)]
pub struct CacheSizeLimits {
    pub guilds: Option<usize>,
    pub channels: Option<usize>,
    pub users: Option<usize>,
    pub members: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientOptions {
    pub intents: u64,
    pub presence: Option<GatewayPresenceUpdateSendData>,
    pub rest: Option<RestOptions>,
    pub gateway_version: Option<String>,
    pub wait_for_guilds: bool,
    pub cache: CacheSizeLimits,
}

pub struct Client {
    pub rest: Rest,
    pub guilds: DashMap<String, Guild>,
    pub channels: DashMap<String, Channel>,
    pub users: DashMap<String, User>,
    pub members: DashMap<String, DashMap<String, GuildMember>>,
    options: ClientOptions,
    handlers: HashMap<String, Vec<EventCallback>>,
    typed_handlers: Vec<TypedEventCallback>,
    ready: bool,
    ready_at: Option<std::time::Instant>,
    user: Option<ClientUser>,
    ws_manager: Option<Arc<RwLock<WebSocketManager>>>,
    expected_guilds: std::collections::HashSet<String>,
    received_guilds: std::collections::HashSet<String>,
    message_collector_senders: Vec<mpsc::UnboundedSender<ApiMessage>>,
    reaction_collector_senders: Vec<mpsc::UnboundedSender<CollectedReaction>>,
    pub voice: Arc<VoiceManager>,
}

impl Client {
    pub fn new(options: ClientOptions) -> Self {
        let rest = Rest::new(options.rest.clone().unwrap_or_default());
        Self {
            rest,
            guilds: DashMap::new(),
            channels: DashMap::new(),
            users: DashMap::new(),
            members: DashMap::new(),
            options,
            handlers: HashMap::new(),
            typed_handlers: Vec::new(),
            ready: false,
            ready_at: None,
            user: None,
            ws_manager: None,
            expected_guilds: std::collections::HashSet::new(),
            received_guilds: std::collections::HashSet::new(),
            message_collector_senders: Vec::new(),
            reaction_collector_senders: Vec::new(),
            voice: Arc::new(VoiceManager::new()),
        }
    }

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

    pub fn on_typed<F, Fut>(&mut self, callback: F)
    where
        F: Fn(DispatchEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let wrapped: TypedEventCallback = Box::new(move |event| Box::pin(callback(event)));
        self.typed_handlers.push(wrapped);
    }

    pub fn user(&self) -> Option<&ClientUser> {
        self.user.as_ref()
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn ready_at(&self) -> Option<std::time::Instant> {
        self.ready_at
    }

    pub fn get_or_create_user(&self, data: &fluxer_types::user::ApiUser) -> User {
        if let Some(mut existing) = self.users.get_mut(&data.id) {
            existing.patch(data);
            return existing.clone();
        }
        let user = User::from_api(data);
        self.users.insert(user.id.clone(), user.clone());
        user
    }

    pub async fn send_to_gateway(&self, payload: Value) {
        if let Some(mgr) = &self.ws_manager {
            mgr.read().await.broadcast(payload).await;
        }
    }

    pub async fn join_voice(
        &self,
        guild_id: &str,
        channel_id: &str,
    ) -> Result<Arc<FluxerVoiceConnection>, VoiceError> {
        self.voice.join(guild_id, channel_id).await
    }

    pub async fn leave_voice(&self, channel_id: &str) -> Result<(), VoiceError> {
        self.voice.disconnect(channel_id).await
    }

    pub async fn send_to_shard(&self, shard_id: u32, payload: Value) -> Result<(), String> {
        if let Some(mgr) = &self.ws_manager {
            return mgr.read().await.send(shard_id, payload).await;
        }
        Err("Not connected".to_string())
    }

    pub async fn fetch_instance(&self) -> crate::Result<Value> {
        let data: Value = self.rest.get(fluxer_types::Routes::instance()).await?;
        Ok(data)
    }

    pub fn create_message_collector(
        &mut self,
        options: MessageCollectorOptions,
    ) -> MessageCollector {
        let (tx, collector) = MessageCollector::new(options);
        self.message_collector_senders.push(tx);
        collector
    }

    pub fn create_reaction_collector(
        &mut self,
        options: ReactionCollectorOptions,
    ) -> ReactionCollector {
        let (tx, collector) = ReactionCollector::new(options);
        self.reaction_collector_senders.push(tx);
        collector
    }

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
            version: self
                .options
                .gateway_version
                .clone()
                .unwrap_or("1".to_string()),
        };

        let mut manager = WebSocketManager::new(ws_options, self.rest.clone(), ws_tx);
        manager.connect().await.map_err(crate::Error::Rest)?;

        self.ws_manager = Some(Arc::new(RwLock::new(manager)));

        let ws_clone = self.ws_manager.as_ref().unwrap().clone();
        self.voice
            .set_gateway_sender(Arc::new(move |payload| {
                let ws = ws_clone.clone();
                tokio::spawn(async move {
                    ws.read().await.broadcast(payload).await;
                });
            }))
            .await;

        while let Some(event) = ws_rx.recv().await {
            match event {
                WsEvent::ShardReady { data, .. } => {
                    if let Some(user_data) = data.get("user")
                        && let Ok(api_user) =
                            serde_json::from_value::<fluxer_types::user::ApiUser>(user_data.clone())
                    {
                        let u = User::from_api(&api_user);
                        self.user = Some(ClientUser::from_user(u.clone()));
                        self.users.insert(api_user.id.clone(), u);
                    }

                    if let Some(guilds_arr) = data.get("guilds").and_then(|v| v.as_array()) {
                        for guild_val in guilds_arr {
                            if let Some(id) = guild_val.get("id").and_then(|v| v.as_str()) {
                                self.expected_guilds.insert(id.to_string());
                            }
                        }
                    }

                    if !self.options.wait_for_guilds || self.expected_guilds.is_empty() {
                        self.ready = true;
                        self.ready_at = Some(std::time::Instant::now());
                        self.emit_event("READY", Value::Null).await;
                        self.emit_typed_event(DispatchEvent::Ready).await;
                    }
                }

                WsEvent::Dispatch { payload, .. } => {
                    if payload.op == GatewayOpcode::Dispatch
                        && let Some(event_name) = &payload.t
                    {
                        let data = payload.d.clone().unwrap_or(Value::Null);
                        self.handle_dispatch(event_name, &data).await;
                        self.enforce_cache_limits();
                        self.emit_event(event_name, data.clone()).await;

                        let typed = event_parser::parse_dispatch(event_name, &data);
                        self.emit_typed_event(typed).await;
                    }
                }

                WsEvent::Error { error, shard_id: _ } => {
                    tracing::error!(target: "fluxer_core::ws", "{error}");
                    self.emit_event("ERROR", Value::String(error.clone())).await;
                    self.emit_typed_event(DispatchEvent::Error { message: error })
                        .await;
                }

                WsEvent::Debug(msg) => {
                    tracing::debug!(target: "fluxer_core::ws", "{msg}");
                    self.emit_event("DEBUG", Value::String(msg.clone())).await;
                    self.emit_typed_event(DispatchEvent::Debug { message: msg })
                        .await;
                }

                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_dispatch(&mut self, event: &str, data: &Value) {
        match event {
            "VOICE_SERVER_UPDATE" => {
                tracing::info!("VOICE_SERVER_UPDATE received: {:?}", data);
                self.voice.handle_voice_server_update(data.clone());
            }

            "MESSAGE_CREATE" => {
                if let Ok(api_msg) = serde_json::from_value::<ApiMessage>(data.clone()) {
                    self.message_collector_senders.retain(|tx| !tx.is_closed());
                    for tx in &self.message_collector_senders {
                        let _ = tx.send(api_msg.clone());
                    }
                }

                if let Some(author) = data.get("author")
                    && let Ok(api_user) =
                        serde_json::from_value::<fluxer_types::user::ApiUser>(author.clone())
                {
                    self.get_or_create_user(&api_user);
                }
                if let Some(member_val) = data.get("member") {
                    let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                    let author_id = data
                        .get("author")
                        .and_then(|a| a.get("id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if !guild_id.is_empty() && !author_id.is_empty() {
                        let mut merged = member_val.clone();
                        if let (Some(obj), Some(au)) = (merged.as_object_mut(), data.get("author"))
                        {
                            obj.insert("user".to_string(), au.clone());
                        }
                        if let Ok(api_member) =
                            serde_json::from_value::<fluxer_types::user::ApiGuildMember>(merged)
                        {
                            let member = GuildMember::from_api(&api_member, guild_id);
                            self.members
                                .entry(guild_id.to_string())
                                .or_default()
                                .insert(author_id.to_string(), member);
                        }
                    }
                }
            }

            "MESSAGE_UPDATE" => {
                if let Some(author) = data.get("author")
                    && let Ok(api_user) =
                        serde_json::from_value::<fluxer_types::user::ApiUser>(author.clone())
                {
                    self.get_or_create_user(&api_user);
                }
            }

            "GUILD_CREATE" => {
                if let Ok(api_guild) =
                    serde_json::from_value::<fluxer_types::guild::ApiGuild>(data.clone())
                {
                    let mut guild = Guild::from_api(&api_guild);

                    if let Some(channels_arr) = data.get("channels").and_then(|v| v.as_array()) {
                        for ch_val in channels_arr {
                            if let Ok(api_ch) = serde_json::from_value::<
                                fluxer_types::channel::ApiChannel,
                            >(ch_val.clone())
                            {
                                guild.channels.push(api_ch.id.clone());
                                let ch = Channel::from_api(&api_ch);
                                self.channels.insert(ch.id.clone(), ch);
                            }
                        }
                    }

                    if let Some(roles_arr) = data.get("roles").and_then(|v| v.as_array()) {
                        for role_val in roles_arr {
                            if let Ok(api_role) = serde_json::from_value::<
                                fluxer_types::role::ApiRole,
                            >(role_val.clone())
                            {
                                let role =
                                    crate::structures::role::Role::from_api(&api_role, &guild.id);
                                guild.roles.insert(role.id.clone(), role);
                            }
                        }
                    }

                    if let Some(members_arr) = data.get("members").and_then(|v| v.as_array()) {
                        let guild_members = self.members.entry(guild.id.clone()).or_default();
                        for m_val in members_arr {
                            if let Ok(api_m) = serde_json::from_value::<
                                fluxer_types::user::ApiGuildMember,
                            >(m_val.clone())
                            {
                                let member = GuildMember::from_api(&api_m, &guild.id);
                                self.users
                                    .entry(member.id.clone())
                                    .or_insert_with(|| member.user.clone());
                                guild_members.insert(member.id.clone(), member);
                            }
                        }
                    }

                    if let Some(mc) = data.get("member_count").and_then(|v| v.as_u64()) {
                        guild.member_count = Some(mc);
                    }

                    let gid = guild.id.clone();
                    self.guilds.insert(gid.clone(), guild);

                    if self.options.wait_for_guilds {
                        self.received_guilds.insert(gid);
                        if self.received_guilds.is_superset(&self.expected_guilds) && !self.ready {
                            self.ready = true;
                            self.ready_at = Some(std::time::Instant::now());
                            self.emit_event("READY", Value::Null).await;
                            self.emit_typed_event(DispatchEvent::Ready).await;
                        }
                    }
                }
            }

            "GUILD_UPDATE" => {
                if let Ok(api_guild) =
                    serde_json::from_value::<fluxer_types::guild::ApiGuild>(data.clone())
                {
                    if let Some(mut g) = self.guilds.get_mut(&api_guild.id) {
                        g.patch(&api_guild);
                    } else {
                        self.guilds
                            .insert(api_guild.id.clone(), Guild::from_api(&api_guild));
                    }
                }
            }

            "GUILD_DELETE" => {
                if let Some(id) = data.get("id").and_then(|v| v.as_str()) {
                    if let Some((_, guild)) = self.guilds.remove(id) {
                        for ch_id in &guild.channels {
                            self.channels.remove(ch_id);
                        }
                    }
                    self.members.remove(id);
                }
            }

            "GUILD_MEMBER_ADD" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                if let Ok(api_m) =
                    serde_json::from_value::<fluxer_types::user::ApiGuildMember>(data.clone())
                {
                    let member = GuildMember::from_api(&api_m, guild_id);
                    if let Some(ref u) = api_m.user {
                        self.get_or_create_user(u);
                    }
                    self.members
                        .entry(guild_id.to_string())
                        .or_default()
                        .insert(member.id.clone(), member);
                    if let Some(mut g) = self.guilds.get_mut(guild_id) {
                        g.member_count = g.member_count.map(|c| c + 1);
                    }
                }
            }

            "GUILD_MEMBER_UPDATE" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                let user_id = data
                    .get("user")
                    .and_then(|u| u.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !guild_id.is_empty()
                    && !user_id.is_empty()
                    && let Some(guild_members) = self.members.get(guild_id)
                    && let Some(mut m) = guild_members.get_mut(user_id)
                {
                    if let Some(nick) = data.get("nick").and_then(|v| v.as_str()) {
                        m.nick = Some(nick.to_string());
                    } else if data.get("nick").map(|v| v.is_null()).unwrap_or(false) {
                        m.nick = None;
                    }
                    if let Some(roles) = data.get("roles").and_then(|v| v.as_array()) {
                        m.role_ids = roles
                            .iter()
                            .filter_map(|r| r.as_str().map(String::from))
                            .collect();
                    }
                }
            }

            "GUILD_MEMBER_REMOVE" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                let user_id = data
                    .get("user")
                    .and_then(|u| u.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !guild_id.is_empty() && !user_id.is_empty() {
                    if let Some(guild_members) = self.members.get(guild_id) {
                        guild_members.remove(user_id);
                    }
                    if let Some(mut g) = self.guilds.get_mut(guild_id) {
                        g.member_count = g.member_count.map(|c| c.saturating_sub(1));
                    }
                }
            }

            "GUILD_ROLE_CREATE" | "GUILD_ROLE_UPDATE" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(role_val) = data.get("role")
                    && let Ok(api_role) =
                        serde_json::from_value::<fluxer_types::role::ApiRole>(role_val.clone())
                    && let Some(mut g) = self.guilds.get_mut(guild_id)
                {
                    let role = crate::structures::role::Role::from_api(&api_role, guild_id);
                    g.roles.insert(role.id.clone(), role);
                }
            }

            "GUILD_ROLE_DELETE" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(role_id) = data.get("role_id").and_then(|v| v.as_str())
                    && let Some(mut g) = self.guilds.get_mut(guild_id)
                {
                    g.roles.remove(role_id);
                }
            }

            "GUILD_EMOJIS_UPDATE" => {
                let guild_id = data.get("guild_id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(mut g) = self.guilds.get_mut(guild_id) {
                    g.emojis.clear();
                    if let Some(arr) = data.get("emojis").and_then(|v| v.as_array()) {
                        for e in arr {
                            if let Some(id) = e.get("id").and_then(|v| v.as_str()) {
                                g.emojis.push(id.to_string());
                            }
                        }
                    }
                }
            }

            "GUILD_BAN_ADD" | "GUILD_BAN_REMOVE" => {
                if let Some(user_val) = data.get("user")
                    && let Ok(api_user) =
                        serde_json::from_value::<fluxer_types::user::ApiUser>(user_val.clone())
                {
                    self.get_or_create_user(&api_user);
                }
            }

            "CHANNEL_CREATE" | "CHANNEL_UPDATE" => {
                if let Ok(api_ch) =
                    serde_json::from_value::<fluxer_types::channel::ApiChannel>(data.clone())
                {
                    let ch = Channel::from_api(&api_ch);
                    if let Some(gid) = &ch.guild_id
                        && let Some(mut g) = self.guilds.get_mut(gid)
                        && !g.channels.contains(&ch.id)
                    {
                        g.channels.push(ch.id.clone());
                    }
                    self.channels.insert(ch.id.clone(), ch);
                }
            }

            "CHANNEL_DELETE" => {
                if let Some(id) = data.get("id").and_then(|v| v.as_str())
                    && let Some((_, ch)) = self.channels.remove(id)
                    && let Some(gid) = &ch.guild_id
                    && let Some(mut g) = self.guilds.get_mut(gid)
                {
                    g.channels.retain(|c| c != id);
                }
            }

            "USER_UPDATE" => {
                if let Ok(api_user) =
                    serde_json::from_value::<fluxer_types::user::ApiUser>(data.clone())
                    && let Some(ref cu) = self.user
                    && cu.id() == api_user.id
                {
                    let u = User::from_api(&api_user);
                    self.user = Some(ClientUser::from_user(u.clone()));
                    self.users.insert(api_user.id.clone(), u);
                }
            }

            "MESSAGE_REACTION_ADD" => {
                self.reaction_collector_senders.retain(|tx| !tx.is_closed());
                if !self.reaction_collector_senders.is_empty() {
                    let reaction = CollectedReaction {
                        message_id: data
                            .get("message_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        channel_id: data
                            .get("channel_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        guild_id: data
                            .get("guild_id")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        user_id: data
                            .get("user_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        emoji_name: data
                            .get("emoji")
                            .and_then(|e| e.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        emoji_id: data
                            .get("emoji")
                            .and_then(|e| e.get("id"))
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        emoji_animated: data
                            .get("emoji")
                            .and_then(|e| e.get("animated"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    };
                    for tx in &self.reaction_collector_senders {
                        let _ = tx.send(reaction.clone());
                    }
                }
            }

            _ => {
                warn!("Unhandled dispatch event: {event}");
            }
        }
    }

    async fn emit_event(&self, event: &str, data: Value) {
        if let Some(handlers) = self.handlers.get(event) {
            for handler in handlers {
                let fut = handler(data.clone());
                tokio::spawn(fut);
            }
        }
    }

    async fn emit_typed_event(&self, event: DispatchEvent) {
        for handler in &self.typed_handlers {
            let fut = handler(event.clone());
            tokio::spawn(fut);
        }
    }

    fn enforce_cache_limits(&self) {
        if let Some(max) = self.options.cache.users {
            while self.users.len() > max {
                if let Some(entry) = self.users.iter().next() {
                    let key = entry.key().clone();
                    drop(entry);
                    self.users.remove(&key);
                } else {
                    break;
                }
            }
        }
        if let Some(max) = self.options.cache.channels {
            while self.channels.len() > max {
                if let Some(entry) = self.channels.iter().next() {
                    let key = entry.key().clone();
                    drop(entry);
                    self.channels.remove(&key);
                } else {
                    break;
                }
            }
        }
        if let Some(max) = self.options.cache.guilds {
            while self.guilds.len() > max {
                if let Some(entry) = self.guilds.iter().next() {
                    let key = entry.key().clone();
                    drop(entry);
                    self.guilds.remove(&key);
                } else {
                    break;
                }
            }
        }
        if let Some(max) = self.options.cache.members {
            while self.members.len() > max {
                if let Some(entry) = self.members.iter().next() {
                    let key = entry.key().clone();
                    drop(entry);
                    self.members.remove(&key);
                } else {
                    break;
                }
            }
        }
    }

    pub fn destroy(&mut self) {
        self.ready = false;
        self.ready_at = None;
        self.user = None;
        self.guilds.clear();
        self.channels.clear();
        self.users.clear();
        self.members.clear();
        self.ws_manager = None;
        self.expected_guilds.clear();
        self.received_guilds.clear();
        self.message_collector_senders.clear();
        self.reaction_collector_senders.clear();
    }
}
