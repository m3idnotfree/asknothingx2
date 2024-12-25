use serde::{Deserialize, Serialize};

mod drop_entitlement_grant_condition;
pub use drop_entitlement_grant_condition::DropEntitlementGrantCondition;

mod channel_raid_condition;
pub use channel_raid_condition::ChannelRaidCondition;

mod conduit_shard_disabled_condition;
pub use conduit_shard_disabled_condition::ConduitShardDisabledCondition;

mod extension_bits_transaction_create_condition;
pub use extension_bits_transaction_create_condition::ExtensionBitsTransactionCreateCondition;

pub trait IntoCondition: Serialize + Sized {
    fn serialize_to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conditions
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl Condition {
    pub fn set_broadcaster_user_id<T: Into<String>>(mut self, broadcaster_user_id: T) -> Self {
        self.broadcaster_user_id = Some(broadcaster_user_id.into());
        self
    }

    pub fn set_moderator_user_id<T: Into<String>>(mut self, moderator_user_id: T) -> Self {
        self.moderator_user_id = Some(moderator_user_id.into());
        self
    }

    pub fn set_broadcaster_id<T: Into<String>>(mut self, broadcaster_id: T) -> Self {
        self.broadcaster_id = Some(broadcaster_id.into());
        self
    }

    pub fn set_user_id<T: Into<String>>(mut self, user_id: T) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn set_reward_id<T: Into<String>>(mut self, reward_id: T) -> Self {
        self.reward_id = Some(reward_id.into());
        self
    }

    pub fn set_client_id<T: Into<String>>(mut self, client_id: T) -> Self {
        self.client_id = Some(client_id.into());
        self
    }
}

impl IntoCondition for Condition {}
impl IntoCondition for DropEntitlementGrantCondition {}
impl IntoCondition for ExtensionBitsTransactionCreateCondition {}
impl IntoCondition for ConduitShardDisabledCondition {}
impl IntoCondition for ChannelRaidCondition {}
