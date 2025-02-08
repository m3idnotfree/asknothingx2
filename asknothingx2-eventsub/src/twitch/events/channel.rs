use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelEvent {
    moderator_user_id: String,
    moderator_user_login: String,
    moderator_user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarningSendEvent {
    pub reason: Option<String>,
    pub chat_rules_cited: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspiciousUserMessageEvent {
    low_trust_status: String,
    shared_ban_channel_ids: Vec<String>,
    types: Vec<String>,
    ban_evasion_evaluation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionMessageEvent {
    tier: String,
}
