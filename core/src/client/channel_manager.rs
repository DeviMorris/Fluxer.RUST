use dashmap::DashMap;

use fluxer_types::channel::ApiChannel;
use fluxer_types::message::ApiMessage;

use crate::structures::channel::Channel;

/// Manages cached channels and provides convenience methods.
///
/// # Examples
/// ```rust,ignore
/// let channel = client.channels.get("123456789");
/// ```
pub struct ChannelManager<'a> {
    cache: &'a DashMap<String, Channel>,
    rest: &'a fluxer_rest::Rest,
}

impl<'a> ChannelManager<'a> {
    pub fn new(cache: &'a DashMap<String, Channel>, rest: &'a fluxer_rest::Rest) -> Self {
        Self { cache, rest }
    }

    /// Get a cached channel by ID.
    pub fn get(&self, id: &str) -> Option<Channel> {
        self.cache.get(id).map(|r| r.clone())
    }

    /// Fetch a channel from the API.
    ///
    /// Caches the result and returns it.
    pub async fn fetch(&self, id: &str) -> crate::Result<Channel> {
        let data: ApiChannel = self
            .rest
            .get(&fluxer_types::Routes::channel(id))
            .await?;
        let channel = Channel::from_api(&data);
        self.cache.insert(channel.id.clone(), channel.clone());
        Ok(channel)
    }

    /// Resolve a channel by ID — from cache or fetching.
    pub async fn resolve(&self, id: &str) -> crate::Result<Channel> {
        if let Some(ch) = self.get(id) {
            return Ok(ch);
        }
        self.fetch(id).await
    }

    /// Send a message to a channel by ID.
    pub async fn send(
        &self,
        channel_id: &str,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = self
            .rest
            .post(
                &fluxer_types::Routes::channel_messages(channel_id),
                Some(body),
            )
            .await?;
        Ok(msg)
    }

    /// Fetch a message from a channel.
    pub async fn fetch_message(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = self
            .rest
            .get(&fluxer_types::Routes::channel_message(channel_id, message_id))
            .await?;
        Ok(msg)
    }
}
