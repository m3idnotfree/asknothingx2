use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::twitch::{subscription_types::SubscriptionType, Transport};

pub type EventPayload<C, E> = SubscriptionEventPayload<C, E>;
pub type SinglePayload<C> = SubscriptionPayload<C>;

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
    pub kind: SubscriptionType,
    pub version: String,
    pub cost: u64,
    pub condition: Condition,
    pub transport: Transport,
    pub created_at: DateTime<FixedOffset>,
}

impl_de_with_subscription_type_must_have_veasion_and_condition!(
    Subscription {
        id: String,
        status: String,
        cost: u64,
        transport: Transport,
        created_at: DateTime<FixedOffset>
    }
);
