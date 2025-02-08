use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#drop-entitlement-grant-event>
#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlementGrantEvent {
    pub id: String,
    pub data: Data,
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#drop-entitlement-grant-event>
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub organization_id: String,
    pub category_id: String,
    pub category_name: String,
    pub campaign_id: String,
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
    pub entitlement_id: String,
    pub benefit_id: String,
    pub created_at: DateTime<FixedOffset>,
}
