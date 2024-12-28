use serde::{Deserialize, Serialize};

use crate::twitch::Transport;

use crate::twitch::IntoCondition;

new_request!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#conduitsharddisabled
    ConduitShardDisabledRequest,
    ConduitShardDisabled,
    ConduitShardDisabledCondition,
    {
        client_id: client_id
    }
);

new_response_payload!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#conduitsharddisabled
    ConduitShardDisabledPayload,
    ConduitShardDisabledCondition,
    ConduitShardDisabledEvent
);

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conduit-shard-disabled-condition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConduitShardDisabledCondition {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conduit_id: Option<String>,
}

impl ConduitShardDisabledCondition {
    pub fn new<T: Into<String>>(client_id: T) -> Self {
        Self {
            client_id: client_id.into(),
            conduit_id: None,
        }
    }

    pub fn set_conduit_id<T: Into<String>>(&mut self, conduit_id: T) {
        self.conduit_id = Some(conduit_id.into());
    }
}

impl IntoCondition for ConduitShardDisabledCondition {}

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conduit-shard-disabled-event
#[derive(Debug, Serialize, Deserialize)]
pub struct ConduitShardDisabledEvent {
    pub conduit_id: String,
    pub shard_id: String,
    pub status: String,
    pub transport: Transport,
}
