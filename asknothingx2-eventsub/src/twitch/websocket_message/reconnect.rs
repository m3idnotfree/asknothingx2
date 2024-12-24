use serde::Serialize;

use crate::twitch::payload::SessionPayload;

use super::MetaData;

/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#reconnect-message
#[derive(Debug, Serialize)]
pub struct Reconnect {
    pub metadata: MetaData,
    pub payload: SessionPayload,
}
