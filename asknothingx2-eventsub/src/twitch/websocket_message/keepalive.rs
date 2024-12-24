use serde::Serialize;
use serde_json::Value;

use super::MetaData;

/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#keepalive-message
#[derive(Debug, Serialize)]
pub struct Keepalive {
    pub metadata: MetaData,
    /// empty object
    pub payload: Value,
}
