use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::timeout;

use fluxer_types::message::ApiMessage;

pub type MessageFilter = Box<dyn Fn(&ApiMessage) -> bool + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndReason {
    Time,
    Limit,
    User,
}

pub struct MessageCollectorOptions {
    pub channel_id: String,
    pub filter: Option<MessageFilter>,
    pub time: Option<Duration>,
    pub max: Option<usize>,
}

pub struct MessageCollector {
    channel_id: String,
    filter: Option<MessageFilter>,
    time: Option<Duration>,
    max: Option<usize>,
    rx: mpsc::UnboundedReceiver<ApiMessage>,
}

impl MessageCollector {
    pub fn new(
        options: MessageCollectorOptions,
    ) -> (mpsc::UnboundedSender<ApiMessage>, Self) {
        let (tx, rx) = mpsc::unbounded_channel();
        let collector = Self {
            channel_id: options.channel_id,
            filter: options.filter,
            time: options.time,
            max: options.max,
            rx,
        };
        (tx, collector)
    }

    pub async fn collect(mut self) -> (Vec<ApiMessage>, EndReason) {
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

            let msg = if let Some(dur) = remaining {
                match timeout(dur, self.rx.recv()).await {
                    Ok(Some(m)) => m,
                    Ok(None) => return (collected, EndReason::User),
                    Err(_) => return (collected, EndReason::Time),
                }
            } else {
                match self.rx.recv().await {
                    Some(m) => m,
                    None => return (collected, EndReason::User),
                }
            };

            if msg.channel_id != self.channel_id {
                continue;
            }

            if let Some(filter) = &self.filter
                && !filter(&msg) {
                    continue;
                }

            collected.push(msg);

            if let Some(max) = self.max
                && collected.len() >= max {
                    return (collected, EndReason::Limit);
                }
        }
    }

    pub fn stop(self) -> (Vec<ApiMessage>, EndReason) {
        (Vec::new(), EndReason::User)
    }
}
