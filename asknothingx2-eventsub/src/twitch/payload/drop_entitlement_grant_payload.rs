//! Note that the payload structure is different from other subscription types. Events bound for drop.entitlement.grant subscriptions are batched. Developers can expect to receive roughly 0-5 HTTP requests per second. HTTP request bodies will not exceed 250KB.
use serde::{Deserialize, Serialize};

use crate::twitch::reference::{
    condition::DropEntitlementGrantCondition, event::DropEntitlementGrantEvent,
};

use super::Subscription;

#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlementGrantPayload {
    pub subscription: Subscription<DropEntitlementGrantCondition>,
    pub events: Vec<DropEntitlementGrantEvent>,
}
