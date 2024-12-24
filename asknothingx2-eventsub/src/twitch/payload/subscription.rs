use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::twitch::{reference::transport::Transport, subscription_type::SubscriptionTypes};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPayload<Condition> {
    pub subscription: Subscription<Condition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionEventPayload<Condition, Event> {
    pub subscription: Subscription<Condition>,
    pub event: Event,
}

#[derive(Debug, Serialize)]
pub struct Subscription<Condition> {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub kind: SubscriptionTypes,
    pub version: String,
    pub cost: u64,
    pub condition: Condition,
    pub transport: Transport,
    pub created_at: DateTime<FixedOffset>,
}

impl<'de, Condition: Deserialize<'de>> Deserialize<'de> for Subscription<Condition> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Helper<Condition> {
            id: String,
            status: String,
            #[serde(rename = "type")]
            kind: SubscriptionTypes,
            version: String,
            cost: u64,
            condition: Condition,
            transport: Transport,
            created_at: DateTime<FixedOffset>,
        }
        let helper = Helper::deserialize(deserializer)?;

        let kind = match helper.kind {
            kind @ SubscriptionTypes::AutomodMessageHold => {
                if helper.version == "2" {
                    SubscriptionTypes::AutomodMessageHoldV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionTypes::AutomodMessageUpdate => {
                if helper.version == "2" {
                    SubscriptionTypes::AutomodMessageUpdateV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionTypes::ChannelModerate => {
                if helper.version == "2" {
                    SubscriptionTypes::ChannelModerateV2
                } else {
                    kind
                }
            }
            _ => helper.kind,
        };

        Ok(Subscription {
            id: helper.id,
            status: helper.status,
            kind,
            version: helper.version,
            cost: helper.cost,
            condition: helper.condition,
            transport: helper.transport,
            created_at: helper.created_at,
        })
    }
}
