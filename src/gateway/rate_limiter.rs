use crate::error::{Result, StateError};
use std::time::{Duration, Instant};
use tokio::sync::Notify;
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutboundKind {
    Normal,
    Internal,
}

#[derive(Debug, Clone)]
pub(crate) struct OutboundRateLimiter {
    commands_per_minute: u32,
    reserved_slots: u32,
    window_start: Instant,
    remaining: i32,
}

impl OutboundRateLimiter {
    pub(crate) fn new(commands_per_minute: u32, reserved_slots: u32) -> Self {
        Self {
            commands_per_minute,
            reserved_slots,
            window_start: Instant::now(),
            remaining: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.window_start = Instant::now();
        self.remaining = self.commands_per_minute as i32;
    }

    pub(crate) async fn wait(&mut self, kind: OutboundKind, shutdown: &Notify) -> Result<()> {
        loop {
            let now = Instant::now();
            if now.duration_since(self.window_start) >= Duration::from_secs(60) {
                self.window_start = now;
                self.remaining = self.commands_per_minute as i32;
            }

            let blocked = self.remaining <= 0
                || (kind == OutboundKind::Normal && self.remaining <= self.reserved_slots as i32);
            if !blocked {
                self.remaining -= 1;
                return Ok(());
            }

            let wait =
                Duration::from_secs(60).saturating_sub(now.duration_since(self.window_start));
            tokio::select! {
                _ = sleep(wait) => {}
                _ = shutdown.notified() => {
                    return Err(StateError::Closed.into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn reserve_slots() {
        let notify = Notify::new();
        let mut limiter = OutboundRateLimiter::new(5, 2);
        limiter.window_start = Instant::now();
        limiter.remaining = 2;

        let res = tokio::time::timeout(
            Duration::from_millis(25),
            limiter.wait(OutboundKind::Normal, &notify),
        )
        .await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn internal_slots() {
        let notify = Notify::new();
        let mut limiter = OutboundRateLimiter::new(5, 2);
        limiter.window_start = Instant::now();
        limiter.remaining = 2;

        limiter
            .wait(OutboundKind::Internal, &notify)
            .await
            .expect("internal slot");
    }

    #[test]
    fn reset_refills() {
        let mut limiter = OutboundRateLimiter::new(7, 2);
        limiter.remaining = 0;
        limiter.reset();
        assert_eq!(limiter.remaining, 7);
    }
}
