use crate::cache::{CachePolicy, CacheUpdater, Caches};
use crate::error::{Result, StateError};
use crate::events::{EventCollector, EventPipeline, EventPipelineConfig};
use crate::gateway::{CompressionMode, GatewayClient, GatewayConfig};
use crate::http::{
    AdminApi, AuthApi, ChannelsApi, GatewayApi, GuildsApi, HttpClient, HttpClientConfig,
    InteractionsApi, InvitesApi, MembersApi, MessagesApi, RolesApi, UsersApi, WebhooksApi,
};
use crate::oauth2::OAuth2Client;
use crate::voice::VoiceClient;
use crate::webhook::WebhookClient;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    Idle,
    Opening,
    Ready,
    Closing,
    Closed,
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub token: Option<String>,
    pub http: HttpClientConfig,
    pub gateway: GatewayConfig,
    pub cache_policy: CachePolicy,
    pub events: EventPipelineConfig,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            token: None,
            http: HttpClientConfig::default(),
            gateway: GatewayConfig::default(),
            cache_policy: CachePolicy::default(),
            events: EventPipelineConfig::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    options: ClientOptions,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        let token = token.into();
        self.options.token = Some(token.clone());
        self.options.http.bot_token = Some(token.clone());
        self.options.gateway.token = token;
        self
    }

    pub fn gateway_url(mut self, url: impl Into<String>) -> Self {
        self.options.gateway.url = url.into();
        self
    }

    pub fn compression(mut self, mode: CompressionMode) -> Self {
        self.options.gateway.compression = mode;
        self
    }

    pub fn http_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.options.http.base_url = base_url.into();
        self
    }

    pub fn cache_policy(mut self, policy: CachePolicy) -> Self {
        self.options.cache_policy = policy;
        self
    }

    pub fn event_pipeline(mut self, cfg: EventPipelineConfig) -> Self {
        self.options.events = cfg;
        self
    }

    pub fn options(mut self, options: ClientOptions) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> Result<Client> {
        Client::from_options(self.options)
    }
}

#[derive(Clone)]
/// High-level Fluxer client that wires HTTP, Gateway, Cache and Event pipeline.
///
/// Build via [`ClientBuilder`] and control runtime lifecycle with [`Client::open`]
/// and [`Client::close`].
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: HttpClient,
    gateway: GatewayClient,
    cache: Arc<Caches>,
    events: EventPipeline,
    state: RwLock<ClientState>,
    bind_task: Mutex<Option<JoinHandle<()>>>,
    cache_updater_task: Mutex<Option<JoinHandle<()>>>,
    cache_auto_update: bool,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn from_options(mut options: ClientOptions) -> Result<Self> {
        let token = options
            .token
            .clone()
            .or_else(|| options.http.bot_token.clone())
            .or_else(|| {
                (!options.gateway.token.is_empty()).then_some(options.gateway.token.clone())
            })
            .ok_or(StateError::Missing("token"))?;

        options.http.bot_token = Some(token.clone());
        options.gateway.token = token;

        let cache_auto_update = options.cache_policy.auto_update;
        let http = HttpClient::new(options.http)?;
        let gateway = GatewayClient::new(options.gateway);
        let cache = Arc::new(Caches::new(options.cache_policy));
        let events = EventPipeline::new(options.events);

        Ok(Self {
            inner: Arc::new(ClientInner {
                http,
                gateway,
                cache,
                events,
                state: RwLock::new(ClientState::Idle),
                bind_task: Mutex::new(None),
                cache_updater_task: Mutex::new(None),
                cache_auto_update,
            }),
        })
    }

    pub fn http(&self) -> &HttpClient {
        &self.inner.http
    }

    pub fn gateway(&self) -> &GatewayClient {
        &self.inner.gateway
    }

    pub fn cache(&self) -> Arc<Caches> {
        self.inner.cache.clone()
    }

    pub fn events(&self) -> &EventPipeline {
        &self.inner.events
    }

    pub fn collector(&self) -> EventCollector {
        self.inner.events.collector()
    }

    pub fn oauth2(&self) -> OAuth2Client {
        OAuth2Client::new(self.inner.http.clone())
    }

    pub fn webhooks(&self) -> WebhookClient {
        WebhookClient::new(self.inner.http.clone())
    }

    pub fn webhooks_api(&self) -> WebhooksApi {
        WebhooksApi::new(self.inner.http.clone())
    }

    pub fn voice(&self) -> VoiceClient {
        VoiceClient::new(self.inner.http.clone(), self.inner.gateway.clone())
    }

    pub fn messages(&self) -> MessagesApi {
        MessagesApi::new(self.inner.http.clone())
    }

    pub fn channels(&self) -> ChannelsApi {
        ChannelsApi::new(self.inner.http.clone())
    }

    pub fn guilds(&self) -> GuildsApi {
        GuildsApi::new(self.inner.http.clone())
    }

    pub fn members(&self) -> MembersApi {
        MembersApi::new(self.inner.http.clone())
    }

    pub fn roles(&self) -> RolesApi {
        RolesApi::new(self.inner.http.clone())
    }

    pub fn users(&self) -> UsersApi {
        UsersApi::new(self.inner.http.clone())
    }

    pub fn users_api(&self) -> UsersApi {
        self.users()
    }

    pub fn auth(&self) -> AuthApi {
        AuthApi::new(self.inner.http.clone())
    }

    pub fn admin(&self) -> AdminApi {
        AdminApi::new(self.inner.http.clone())
    }

    pub fn invites(&self) -> InvitesApi {
        InvitesApi::new(self.inner.http.clone())
    }

    pub fn gateway_api(&self) -> GatewayApi {
        GatewayApi::new(self.inner.http.clone())
    }

    pub fn interactions(&self) -> InteractionsApi {
        InteractionsApi::new(self.inner.http.clone())
    }

    pub async fn state(&self) -> ClientState {
        *self.inner.state.read().await
    }

    pub async fn open(&self) -> Result<()> {
        {
            let mut state = self.inner.state.write().await;
            match *state {
                ClientState::Ready | ClientState::Opening => return Ok(()),
                ClientState::Closing => {
                    return Err(StateError::InvalidTransition {
                        from: "closing",
                        to: "open",
                    }
                    .into());
                }
                ClientState::Closed | ClientState::Idle => *state = ClientState::Opening,
            }
        }

        self.inner.gateway.open().await?;
        if self.inner.bind_task.lock().await.is_none() {
            let handle = self.inner.events.bind_gateway(&self.inner.gateway);
            *self.inner.bind_task.lock().await = Some(handle);
        }
        if self.inner.cache_auto_update && self.inner.cache_updater_task.lock().await.is_none() {
            let updater = CacheUpdater::new(self.inner.cache.clone());
            let handle = updater.spawn(self.inner.events.subscribe());
            *self.inner.cache_updater_task.lock().await = Some(handle);
        }

        *self.inner.state.write().await = ClientState::Ready;
        Ok(())
    }

    pub async fn close(&self) {
        {
            let mut state = self.inner.state.write().await;
            if matches!(*state, ClientState::Closed | ClientState::Closing) {
                return;
            }
            *state = ClientState::Closing;
        }

        self.inner.gateway.close().await;
        self.inner.events.close().await;
        self.inner.http.shutdown().await;

        if let Some(handle) = self.inner.bind_task.lock().await.take() {
            handle.abort();
            if let Err(err) = handle.await {
                if err.is_panic() {
                    std::panic::resume_unwind(err.into_panic());
                }
            }
        }
        if let Some(handle) = self.inner.cache_updater_task.lock().await.take() {
            handle.abort();
            if let Err(err) = handle.await {
                if err.is_panic() {
                    std::panic::resume_unwind(err.into_panic());
                }
            }
        }

        *self.inner.state.write().await = ClientState::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_token() {
        let client = Client::builder().token("abc").build().expect("build");
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("rt");
        let state = rt.block_on(client.state());
        assert_eq!(state, ClientState::Idle);
    }

    #[test]
    fn missing_token() {
        let err = match Client::builder().build() {
            Ok(_) => panic!("expected error"),
            Err(err) => err,
        };
        match err {
            crate::error::Error::State(StateError::Missing("token")) => {}
            _ => panic!("wrong error"),
        }
    }

    #[test]
    fn cache_auto_update_default() {
        let client = Client::builder().token("abc").build().expect("build");
        assert!(client.inner.cache_auto_update);
    }

    #[test]
    fn cache_auto_update_off() {
        let policy = CachePolicy {
            auto_update: false,
            ..CachePolicy::default()
        };
        let client = Client::builder()
            .token("abc")
            .cache_policy(policy)
            .build()
            .expect("build");
        assert!(!client.inner.cache_auto_update);
    }
}
