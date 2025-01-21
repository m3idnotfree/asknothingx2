use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

use crate::twitch::{
    subscription::Subscription,
    subscription_types::{request::IntoSubscriptionRequest, types::SubscriptionType},
    Transport,
};

mod condition;
mod event;

pub use condition::DropEntitlementGrantCondition;
pub use event::{DEGEData, DropEntitlementGrantEvent};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#dropentitlementgrant>
#[derive(Debug, Serialize)]
pub struct DropEntitlementGrantRequest {
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: DropEntitlementGrantCondition,
    pub transport: Transport,
    pub scope: HashSet<String>,
    pub is_batching_enabled: String,
}

impl DropEntitlementGrantRequest {
    pub fn websocket<T: Into<String>>(organization_id: T, session_id: T) -> Self {
        let kind = SubscriptionType::DropEntitlementGrant;
        Self {
            version: kind.version().to_string(),
            kind,
            condition: DropEntitlementGrantCondition::new(organization_id.into()),
            transport: Transport::websocket(session_id.into()),
            scope: HashSet::new(),
            is_batching_enabled: "true".to_string(),
        }
    }

    pub fn webhook<T: Into<String>>(organization_id: T, callback: T, secret: Option<T>) -> Self {
        let kind = SubscriptionType::DropEntitlementGrant;
        Self {
            version: kind.version().to_string(),
            kind,
            condition: DropEntitlementGrantCondition::new(organization_id.into()),
            transport: Transport::webhook(callback.into(), secret.map(Into::into)),
            scope: HashSet::new(),
            is_batching_enabled: "true".to_string(),
        }
    }

    pub fn set_category_id<T: Into<String>>(mut self, category_id: T) -> Self {
        self.condition.category_id = Some(category_id.into());
        self
    }

    pub fn set_campaign_id<T: Into<String>>(mut self, campaign_id: T) -> Self {
        self.condition.campaign_id = Some(campaign_id.into());
        self
    }

    pub fn set_require<T, L>(mut self, scopes: L) -> Self
    where
        T: Into<String> + Eq + Hash,
        L: IntoIterator<Item = T>,
    {
        self.scope.extend(scopes.into_iter().map(Into::into));

        self
    }
}

impl_de_without_generic_subscription_type_must_have_veasion_and_condition!(
    DropEntitlementGrantRequest {
        condition: DropEntitlementGrantCondition,
        transport: Transport,
        scope: HashSet<String>,
        is_batching_enabled: String
    }
);

impl IntoSubscriptionRequest for DropEntitlementGrantRequest {}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#dropentitlementgrant>
#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlementGrantPayload {
    pub subscription: Subscription<DropEntitlementGrantCondition>,
    pub events: Vec<DropEntitlementGrantEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropEntitlementGrantPayloadNotification {
    #[serde(flatten)]
    pub payload: DropEntitlementGrantPayload,
}
