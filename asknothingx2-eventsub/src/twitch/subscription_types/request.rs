use std::{collections::HashSet, hash::Hash};

use serde::Serialize;

use crate::twitch::reference::{
    condition::{DropEntitlementGrantCondition, IntoCondition},
    transport::Transport,
};

use super::types::SubscriptionTypes;

pub trait IntoSubscriptionRequest: Serialize + Sized {
    fn into_body(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn check_scope<T: Into<String> + Eq + Hash, L: IntoIterator<Item = T>>(&self, scope: L)
        -> bool;
}

impl<Condition> IntoSubscriptionRequest for SubscriptionRequest<Condition>
where
    Condition: Serialize,
{
    fn check_scope<T: Into<String> + Eq + Hash, L: IntoIterator<Item = T>>(
        &self,
        scope: L,
    ) -> bool {
        !scope
            .into_iter()
            .map(Into::into)
            .collect::<HashSet<String>>()
            .is_disjoint(&self.scope)
    }
}

#[derive(Debug, Serialize)]
pub struct SubscriptionRequest<Condition> {
    #[serde(rename = "type")]
    pub kind: SubscriptionTypes,
    pub version: String,
    pub condition: Condition,
    pub transport: Transport,
    pub scope: HashSet<String>,
}

impl<Condition> SubscriptionRequest<Condition>
where
    Condition: IntoCondition,
{
    pub fn new(kind: SubscriptionTypes, condition: Condition, transport: Transport) -> Self {
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
            scope: HashSet::new(),
        }
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

impl_de_with_subscription_type_must_have_veasion_and_condition!(
    SubscriptionRequest {
        transport: Transport,
        scope: HashSet<String>
    }
);

#[derive(Debug, Serialize)]
pub struct DropEntitlementGrantRequest {
    #[serde(rename = "type")]
    pub kind: SubscriptionTypes,
    pub version: String,
    pub condition: DropEntitlementGrantCondition,
    pub transport: Transport,
    pub scope: HashSet<String>,
    pub is_batching_enabled: String,
}

impl DropEntitlementGrantRequest {
    pub fn new(condition: DropEntitlementGrantCondition, transport: Transport) -> Self {
        let kind = SubscriptionTypes::DropEntitlementGrant;
        Self {
            version: kind.version().to_string(),
            kind,
            condition,
            transport,
            scope: HashSet::new(),
            is_batching_enabled: "true".to_string(),
        }
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

impl IntoSubscriptionRequest for DropEntitlementGrantRequest {
    fn check_scope<T: Into<String> + Eq + Hash, L: IntoIterator<Item = T>>(
        &self,
        scope: L,
    ) -> bool {
        !scope
            .into_iter()
            .map(Into::into)
            .collect::<HashSet<String>>()
            .is_disjoint(&self.scope)
    }
}
