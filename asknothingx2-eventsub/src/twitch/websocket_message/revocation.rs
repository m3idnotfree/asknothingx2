use serde::Serialize;

use crate::twitch::payload::SubscriptionPayload;

use super::MetaData;

/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#revocation-message
#[derive(Debug, Serialize)]
pub struct Revocation<Condition> {
    pub metadata: MetaData,
    pub payload: SubscriptionPayload<Condition>,
}
