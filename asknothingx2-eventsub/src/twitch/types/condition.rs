use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};
use twitch_highway::types::{BroadcasterId, ModeratorId, RewardId, UserId};

use super::{
    new_types::{CampaignId, CategoryId, OrganizationId},
    ConduitId, ExtensionClientId,
};

macro_rules! condition {
    (
    $(#[$attr:meta])*
    $name:ident {
    $(
        $(#[$type_attr:meta])*
        $vis:vis $field:ident: $type:ty
    ),* $(,)?
    }
) => {
    $(#[$attr])*
    pub struct $name {
    $(
        #[serde(skip_serializing_if = "Option::is_none")]
        $(#[$type_attr])*
        $vis $field: Option<$type>
    ),*
    }

    impl $name {
        pub fn new() -> Self {
            Self::default()
        }

        $(
            $(#[$type_attr])*
            pub fn $field(mut self, value: $type) -> Self {
                self.$field = Some(value);
                self
            }
        )*
    }
};
}
condition!(
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conditions>
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    Condition {
        pub broadcaster_user_id: BroadcasterId,
        pub moderator_user_id: ModeratorId,
        pub broadcaster_id: BroadcasterId,
        pub user_id: UserId,
        pub client_id: ClientId,

        /// Channel Raid Condition
        #[cfg(feature ="twitch-raid")]
        pub from_broadcaster_user_id: BroadcasterId,
        /// Channel Raid Condition
        #[cfg(feature ="twitch-raid")]
        pub to_broadcaster_user_id: BroadcasterId,

        #[cfg(feature ="twitch-reward")]
        pub reward_id: RewardId,

        /// Conduit Shard Disabled Condition
        #[cfg(feature ="twitch-conduit-shard")]
        pub conduit_id: ConduitId,

        /// Drop Entitlement Grant Condition
        #[cfg(feature ="twitch-drop-entitlement")]
        pub organization_id: OrganizationId,
        /// Drop Entitlement Grant Condition
        #[cfg(feature ="twitch-drop-entitlement")]
        pub category_id: CategoryId,
        /// Drop Entitlement Grant Condition
        #[cfg(feature ="twitch-drop-entitlement")]
        pub campaign_id: CampaignId,

        /// Extension Bits Transaction Create Condition
        #[cfg(feature="twitch-bits-transaction")]
        pub extension_client_id: ExtensionClientId,
    }
);
