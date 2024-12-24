use serde::{Deserialize, Serialize};

mod drop_entitlement_drang_condition;
pub use drop_entitlement_drang_condition::DropEntitlementGrantCondition;

mod channel_raid_condition;
pub use channel_raid_condition::ChannelRaidCondition;

mod conduit_shard_disabled_condition;
pub use conduit_shard_disabled_condition::ConduitShardDisabledCondition;

mod extension_bits_transaction_create_condition;
pub use extension_bits_transaction_create_condition::ExtensionBitsTransactionCreateCondition;

pub trait GeneralCondition {}

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

impl GeneralCondition for Condition {}
impl GeneralCondition for DropEntitlementGrantCondition {}
impl GeneralCondition for ExtensionBitsTransactionCreateCondition {}
impl GeneralCondition for ConduitShardDisabledCondition {}
impl GeneralCondition for ChannelRaidCondition {}
