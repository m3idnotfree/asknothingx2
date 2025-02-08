use serde::{Deserialize, Serialize};

use crate::twitch::subscription_types::user::authorization_grant_event::AuthorizationGrantEvent;

use super::subscription::Subscription;

pub type EventPayload<E> = SubscriptionEventPayload<E>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPayload {
    pub subscription: Subscription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionEventPayload<Event> {
    pub subscription: Subscription,
    pub event: Event,
}

pub type UserAuthorizationRevokeNotification = SubscriptionEventPayload<AuthorizationGrantEvent>;
