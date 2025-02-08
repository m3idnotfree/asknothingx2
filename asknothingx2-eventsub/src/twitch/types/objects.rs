use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use twitch_highway::types::{RewardId, UserId};

/// NOTE: Bits voting is not supported.
#[derive(Debug, Serialize, Deserialize)]
pub struct BitsVoting {
    /// Not used; will be set to false.
    pub is_enabled: bool,
    /// Not used; will be set to 0.
    pub amount_per_vote: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelPointsVoting {
    pub is_enabled: bool,
    pub amount_per_vote: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub id: String,
    pub title: String,
    /// Not used; will be set to 0.
    pub bits_votes: u8,
    pub channel_points_votes: u64,
    pub votes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emotes {
    begin: u64,
    end: u64,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCooldown {
    pub is_enabled: bool,
    pub seconds: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Contribution {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    #[serde(rename = "type")]
    pub kind: LastContributionType,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LastContributionType {
    Bits,
    Subscription,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxPerStream {
    pub is_enabled: bool,
    pub value: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub emotes: Vec<Emotes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcomes {
    pub id: String,
    pub title: String,
    pub color: String,
    pub users: u64,
    pub channel_points: u64,
    pub top_predictors: Vec<TopPredictor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TopPredictor {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub channel_points_won: Option<u64>,
    pub channel_points_used: u64,
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#product>
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub bits: u64,
    pub sku: String,
    /// Flag indicating if the product is in development. If in_development is true, bits will be 0.
    pub in_development: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reward {
    pub id: RewardId,
    pub title: String,
    pub cost: u64,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldMode {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: String,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoutoutCreate {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub to_broadcaster_user_id: String,
    pub to_broadcaster_user_login: String,
    pub to_broadcaster_user_name: String,
    pub moderator_user_id: String,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub viewer_count: u64,
    pub started_at: DateTime<FixedOffset>,
    pub cooldown_ends_at: DateTime<FixedOffset>,
    pub target_cooldown_ends_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoutoutReceived {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub from_broadcaster_user_id: String,
    pub from_broadcaster_user_login: String,
    pub from_broadcaster_user_name: String,
    pub viewer_count: u64,
    pub started_at: DateTime<FixedOffset>,
}
