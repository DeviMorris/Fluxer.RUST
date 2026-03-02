use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::timeout;

use super::message_collector::EndReason;

pub type ReactionFilter = Box<dyn Fn(&CollectedReaction) -> bool + Send + Sync>;

#[derive(Debug, Clone)]
pub struct CollectedReaction {
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub user_id: String,
    pub emoji_name: String,
    pub emoji_id: Option<String>,
    pub emoji_animated: bool,
}

impl CollectedReaction {
    pub fn key(&self) -> String {
        match &self.emoji_id {
            Some(id) => format!("{}:{}:{}", self.user_id, self.emoji_name, id),
            None => format!("{}:{}", self.user_id, self.emoji_name),
        }
    }
}

pub struct ReactionCollectorOptions {
    pub message_id: String,
    pub channel_id: String,
    pub filter: Option<ReactionFilter>,
    pub time: Option<Duration>,
    pub max: Option<usize>,
}

pub struct ReactionCollector {
    message_id: String,
    channel_id: String,
    filter: Option<ReactionFilter>,
    time: Option<Duration>,
    max: Option<usize>,
    rx: mpsc::UnboundedReceiver<CollectedReaction>,
}

impl ReactionCollector {
    pub fn new(
        options: ReactionCollectorOptions,
    ) -> (mpsc::UnboundedSender<CollectedReaction>, Self) {
        let (tx, rx) = mpsc::unbounded_channel();
        let collector = Self {
            message_id: options.message_id,
            channel_id: options.channel_id,
            filter: options.filter,
            time: options.time,
            max: options.max,
            rx,
        };
        (tx, collector)
    }

    pub async fn collect(mut self) -> (Vec<CollectedReaction>, EndReason) {
        let mut collected = Vec::new();

        let deadline = self.time.map(|d| tokio::time::Instant::now() + d);

        loop {
            let remaining = deadline.map(|d| {
                d.checked_duration_since(tokio::time::Instant::now())
                    .unwrap_or(Duration::ZERO)
            });

            if let Some(Duration::ZERO) = remaining {
                return (collected, EndReason::Time);
            }

            let reaction = if let Some(dur) = remaining {
                match timeout(dur, self.rx.recv()).await {
                    Ok(Some(r)) => r,
                    Ok(None) => return (collected, EndReason::User),
                    Err(_) => return (collected, EndReason::Time),
                }
            } else {
                match self.rx.recv().await {
                    Some(r) => r,
                    None => return (collected, EndReason::User),
                }
            };

            if reaction.message_id != self.message_id || reaction.channel_id != self.channel_id {
                continue;
            }

            if let Some(filter) = &self.filter
                && !filter(&reaction) {
                    continue;
                }

            collected.push(reaction);

            if let Some(max) = self.max
                && collected.len() >= max {
                    return (collected, EndReason::Limit);
                }
        }
    }
}
