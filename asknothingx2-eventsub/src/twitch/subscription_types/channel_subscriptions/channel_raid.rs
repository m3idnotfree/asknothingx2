use serde::{Deserialize, Serialize};

use crate::twitch::IntoCondition;
new_request!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelraid
    ChannelRaidRequest, 
    ChannelRaid,
    ChannelRaidCondition);

impl ChannelRaidRequest {
    pub fn set_from_broadcaster_user_id<T: Into<String>>(
        mut self,
        from_broadcaster_user_id: T,
    ) -> Self {
        self.0.condition.from_broadcaster_user_id = Some(from_broadcaster_user_id.into());
        self
    }

    pub fn set_to_broadcacter_user_id<T: Into<String>>(
        mut self,
        to_broadcaster_user_id: T,
    ) -> Self {
        self.0.condition.to_broadcaster_user_id = Some(to_broadcaster_user_id.into());
        self
    }
}

new_response_payload!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelraid
    ChannelRaidPayload,
    ChannelRaidCondition,
    ChannelRaidEvent
);

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#channel-raid-condition
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChannelRaidCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_broadcaster_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_broadcaster_user_id: Option<String>,
}

impl ChannelRaidCondition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_from_broadcaster_user_id<T: Into<String>>(&mut self, from_broadcaster_user_id: T) {
        self.from_broadcaster_user_id = Some(from_broadcaster_user_id.into());
    }

    pub fn set_to_broadcacter_user_id<T: Into<String>>(&mut self, to_broadcaster_user_id: T) {
        self.to_broadcaster_user_id = Some(to_broadcaster_user_id.into());
    }
}

impl IntoCondition for ChannelRaidCondition {}

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#channel-raid-event
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelRaidEvent {
    pub from_broadcaster_user_id: String,
    pub from_broadcaster_user_login: String,
    pub from_broadcaster_user_name: String,
    pub to_broadcaster_user_id: String,
    pub to_broadcaster_user_login: String,
    pub to_broadcaster_user_name: String,
    pub viewers: u64,
}
