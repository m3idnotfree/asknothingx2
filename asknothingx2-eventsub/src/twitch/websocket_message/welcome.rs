//! IMPORTANT
//! By default, you have 10 seconds from the time you receive the Welcome message to subscribe to an event,
//! unless otherwise specified when connecting.
//! If you don’t subscribe within this timeframe,
//! the server closes the connection.

use serde::Serialize;

use crate::twitch::payload::SessionPayload;

use super::MetaData;

/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#welcome-message
#[derive(Debug, Serialize)]
pub struct Welcome {
    pub metadata: MetaData,
    pub payload: SessionPayload,
}
