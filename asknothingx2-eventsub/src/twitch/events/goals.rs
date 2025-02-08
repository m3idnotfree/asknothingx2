use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalsEvent {
    #[serde(rename = "type")]
    pub kind: GoalType,
    pub description: String,
    pub is_achieved: bool,
    pub current_amount: u64,
    pub target_amount: u64,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Follow,
    Subscription,
    SubscriptionCount,
    NewSubscription,
    NewSubscriptionCount,
    NewBit,
    NewCheerer,
}
