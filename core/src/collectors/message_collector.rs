use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::timeout;

use fluxer_types::message::ApiMessage;

/// Why the collector stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndReason {
    Time,
    Limit,
    User,
}

/// Options for the message collector.
pub struct MessageCollectorOptions {
    pub channel_id: String,
    pub filter: Option<Box<dyn Fn(&ApiMessage) -> bool + Send + Sync>>,
    pub time: Option<Duration>,
    pub max: Option<usize>,
}

/// Collects messages in a channel.
///
/// Receives messages through an `mpsc` channel and applies
/// the configured filter, time limit, and max count.
///
/// # Examples
/// ```rust,ignore
/// let (tx, collector) = MessageCollector::new(MessageCollectorOptions {
///     channel_id: "123".into(),
///     filter: Some(Box::new(|m| !m.author.bot.unwrap_or(false))),
///     time: Some(Duration::from_secs(30)),
///     max: Some(10),
/// });
///
/// // Feed messages from the event loop
/// tx.send(message_data).ok();
///
/// // Await results
/// let (collected, reason) = collector.collect().await;
/// ```
pub struct MessageCollector {
    channel_id: String,
    filter: Option<Box<dyn Fn(&ApiMessage) -> bool + Send + Sync>>,
    time: Option<Duration>,
    max: Option<usize>,
    rx: mpsc::UnboundedReceiver<ApiMessage>,
}

impl MessageCollector {
    /// Create a collector and the sender to feed messages to it.
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

    /// Run the collector until it ends, returning collected messages and the reason.
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

            if let Some(filter) = &self.filter {
                if !filter(&msg) {
                    continue;
                }
            }

            collected.push(msg);

            if let Some(max) = self.max {
                if collected.len() >= max {
                    return (collected, EndReason::Limit);
                }
            }
        }
    }

    /// Stop the collector manually (drop the sender).
    pub fn stop(self) -> (Vec<ApiMessage>, EndReason) {
        (Vec::new(), EndReason::User)
    }
}
