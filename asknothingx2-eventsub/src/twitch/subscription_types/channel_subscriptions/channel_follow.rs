use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::twitch::Condition;

new_request!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow
/// channel.follow
    ChannelFollowRequest,
    ChannelFollow,
    {
        broadcaster_user_id: broadcaster_user_id,
        moderator_user_id: moderator_user_id
    }
);
new_response_payload!(
/// https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow
    ChannelFollowPayload,
    Condition,
    ChannelFollowEvent
);

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#channel-follow-event
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelFollowEvent {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub followed_at: DateTime<FixedOffset>,
}
