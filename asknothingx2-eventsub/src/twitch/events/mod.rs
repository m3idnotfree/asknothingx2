use serde::{Deserialize, Serialize};
use twitch_highway::types::{Id, UserId};

use super::types::BroadcasterUserId;

pub mod automod;
pub mod bits_transaction;
pub mod channel;
pub mod charity;
pub mod conduit_shard;
pub mod drop_entitlement;
pub mod goals;
pub mod hype_train;
pub mod stream;
pub mod types;
pub mod user;
pub mod whisper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Id>,
    pub broadcaster_user_id: BroadcasterUserId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEvent {
    pub user_id: UserId,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
}
