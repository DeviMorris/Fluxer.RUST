use crate::error::{Result, StateError};
use crate::gateway::{DispatchEnvelope, GatewayClient};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Notify, RwLock, broadcast, mpsc};
use tokio::task::JoinHandle;

type HandlerFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
type HandlerFn = Arc<dyn Fn(DispatchEnvelope) -> HandlerFuture + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchStrategy {
    Sequential,
    Concurrent,
}

#[derive(Debug, Clone)]
pub struct EventPipelineConfig {
    pub queue_size: usize,
    pub listeners_size: usize,
    pub strategy: DispatchStrategy,
}

impl Default for EventPipelineConfig {
    fn default() -> Self {
        Self {
            queue_size: 512,
            listeners_size: 512,
            strategy: DispatchStrategy::Concurrent,
        }
    }
}

#[derive(Clone)]
pub struct EventPipeline {
    inner: Arc<Inner>,
}

struct Inner {
    strategy: DispatchStrategy,
    handlers: RwLock<Vec<HandlerFn>>,
    listeners_tx: broadcast::Sender<DispatchEnvelope>,
    ingest_tx: mpsc::Sender<DispatchEnvelope>,
    shutdown: Notify,
    closed: AtomicBool,
    worker: std::sync::Mutex<Option<JoinHandle<()>>>,
}

impl EventPipeline {
    pub fn new(cfg: EventPipelineConfig) -> Self {
        let (listeners_tx, _) = broadcast::channel(cfg.listeners_size);
        let (ingest_tx, ingest_rx) = mpsc::channel(cfg.queue_size);
        let inner = Arc::new(Inner {
            strategy: cfg.strategy,
            handlers: RwLock::new(Vec::new()),
            listeners_tx,
            ingest_tx,
            shutdown: Notify::new(),
            closed: AtomicBool::new(false),
            worker: std::sync::Mutex::new(None),
        });

        let worker_inner = inner.clone();
        let worker = tokio::spawn(async move {
            run_worker(worker_inner, ingest_rx).await;
        });
        if let Ok(mut slot) = inner.worker.lock() {
            *slot = Some(worker);
        }

        Self { inner }
    }

    pub async fn add_handler<F, Fut>(&self, f: F)
    where
        F: Fn(DispatchEnvelope) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handler: HandlerFn = Arc::new(move |event| Box::pin(f(event)));
        self.inner.handlers.write().await.push(handler);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DispatchEnvelope> {
        self.inner.listeners_tx.subscribe()
    }

    pub fn collector(&self) -> EventCollector {
        EventCollector {
            rx: self.subscribe(),
        }
    }

    pub async fn ingest(&self, event: DispatchEnvelope) -> Result<()> {
        if self.inner.closed.load(Ordering::SeqCst) {
            return Err(StateError::Closed.into());
        }
        self.inner
            .ingest_tx
            .send(event)
            .await
            .map_err(|_| StateError::Closed.into())
    }

    pub fn bind_gateway(&self, gateway: &GatewayClient) -> JoinHandle<()> {
        let mut rx = gateway.subscribe_dispatch();
        let pipeline = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = pipeline.inner.shutdown.notified() => return,
                    msg = rx.recv() => {
                        match msg {
                            Ok(event) => {
                                if pipeline.ingest(event).await.is_err() {
                                    return;
                                }
                            }
                            Err(broadcast::error::RecvError::Closed) => return,
                            Err(broadcast::error::RecvError::Lagged(_)) => {}
                        }
                    }
                }
            }
        })
    }

    pub async fn close(&self) {
        self.inner.closed.store(true, Ordering::SeqCst);
        self.inner.shutdown.notify_waiters();
        let handle = if let Ok(mut slot) = self.inner.worker.lock() {
            slot.take()
        } else {
            None
        };
        if let Some(handle) = handle {
            let _ = handle.await;
        }
    }
}

pub struct EventCollector {
    rx: broadcast::Receiver<DispatchEnvelope>,
}

impl EventCollector {
    pub async fn next(&mut self) -> Option<DispatchEnvelope> {
        loop {
            match self.rx.recv().await {
                Ok(ev) => return Some(ev),
                Err(broadcast::error::RecvError::Closed) => return None,
                Err(broadcast::error::RecvError::Lagged(_)) => {}
            }
        }
    }

    pub async fn next_kind(&mut self, kind: &str) -> Option<DispatchEnvelope> {
        while let Some(ev) = self.next().await {
            if ev.event.kind() == kind {
                return Some(ev);
            }
        }
        None
    }
}

async fn run_worker(inner: Arc<Inner>, mut rx: mpsc::Receiver<DispatchEnvelope>) {
    loop {
        tokio::select! {
            _ = inner.shutdown.notified() => return,
            msg = rx.recv() => {
                let Some(event) = msg else {
                    return;
                };
                dispatch_one(&inner, event).await;
            }
        }
    }
}

async fn dispatch_one(inner: &Arc<Inner>, event: DispatchEnvelope) {
    let handlers = inner.handlers.read().await.clone();
    let _ = inner.listeners_tx.send(event.clone());

    match inner.strategy {
        DispatchStrategy::Sequential => {
            for handler in handlers {
                handler(event.clone()).await;
            }
        }
        DispatchStrategy::Concurrent => {
            for handler in handlers {
                let ev = event.clone();
                tokio::spawn(async move {
                    handler(ev).await;
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::{DispatchEvent, UnknownDispatchEvent};
    use serde_json::json;
    use tokio::sync::oneshot;

    fn ready(seq: u64) -> DispatchEnvelope {
        DispatchEnvelope {
            seq,
            event: DispatchEvent::Unknown(UnknownDispatchEvent {
                event_type: "READY".to_owned(),
                raw: json!({"session_id":"s"}),
            }),
        }
    }

    #[tokio::test]
    async fn handler_runs() {
        let p = EventPipeline::new(EventPipelineConfig::default());
        let (tx, rx) = oneshot::channel();
        let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

        p.add_handler({
            let tx = tx.clone();
            move |_| {
                let tx = tx.clone();
                async move {
                    if let Ok(mut guard) = tx.lock() {
                        if let Some(tx) = guard.take() {
                            let _ = tx.send(());
                        }
                    }
                }
            }
        })
        .await;

        p.ingest(ready(1)).await.expect("ingest");
        let _ = tokio::time::timeout(std::time::Duration::from_millis(200), rx)
            .await
            .expect("handler");
        p.close().await;
    }

    #[tokio::test]
    async fn collector_next_kind() {
        let p = EventPipeline::new(EventPipelineConfig::default());
        let mut c = p.collector();
        p.ingest(ready(2)).await.expect("ingest");
        let ev = c.next_kind("READY").await.expect("event");
        assert_eq!(ev.seq, 2);
        p.close().await;
    }
}
