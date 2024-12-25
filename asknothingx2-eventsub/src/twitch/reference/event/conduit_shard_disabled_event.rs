use serde::{Deserialize, Serialize};

use crate::twitch::reference::transport::Transport;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitShardDisabledEvent {
    pub conduit_id: String,
    pub shard_id: String,
    pub status: String,
    pub transport: Transport,
}
