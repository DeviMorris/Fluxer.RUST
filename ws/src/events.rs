use serde_json::Value;
use fluxer_types::gateway::GatewayReceivePayload;

#[derive(Debug, Clone)]
pub enum ShardEvent {
    Ready(Value),
    Resumed,
    Dispatch(GatewayReceivePayload),
    Close(u16),
    Error(String),
    Debug(String),
}

#[derive(Debug, Clone)]
pub enum WsEvent {
    ShardReady {
        shard_id: u32,
        data: Value,
    },
    ShardResumed {
        shard_id: u32,
    },
    Dispatch {
        shard_id: u32,
        payload: GatewayReceivePayload,
    },
    ShardClose {
        shard_id: u32,
        code: u16,
    },
    Error {
        shard_id: u32,
        error: String,
    },
    Debug(String),
}
