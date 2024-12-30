use std::{collections::HashSet, hash::Hash};

use serde::Serialize;

use crate::twitch::{IntoCondition, Transport};

use super::types::SubscriptionType;

pub trait IntoSubscriptionRequest: Serialize + Sized {
    fn into_body(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    // fn check_scope<T: Into<String> + Eq + Hash, L: IntoIterator<Item = T>>(&self, scope: L)
    //     -> bool;
    // fn check_scope<T: Into<String> + Eq + Hash, L: IntoIterator<Item = T>>(
    //     &self,
    //     scope: L,
    // ) -> bool {
    //     !scope
    //         .into_iter()
    //         .map(Into::into)
    //         .collect::<HashSet<String>>()
    //         .is_disjoint(&self.scope)
    // }
}

impl<Condition> IntoSubscriptionRequest for SubscriptionRequest<Condition> where Condition: Serialize
{}

#[derive(Debug, Serialize)]
pub struct SubscriptionRequest<Condition> {
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub condition: Condition,
    pub transport: Transport,
    #[serde(skip)]
    pub scope: HashSet<String>,
}

impl<Condition> SubscriptionRequest<Condition>
where
    Condition: IntoCondition,
{
    pub fn new(kind: SubscriptionType, condition: Condition, transport: Transport) -> Self {
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
