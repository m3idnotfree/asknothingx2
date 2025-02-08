use serde::{Deserialize, Serialize};

use crate::twitch::types::Transport;

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conduit-shard-disabled-event>
#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitShardDisabledEvent {
    pub conduit_id: String,
    pub shard_id: String,
    pub status: String,
    pub transport: Transport,
}
