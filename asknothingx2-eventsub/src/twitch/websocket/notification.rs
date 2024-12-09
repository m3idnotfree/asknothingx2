use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::twitch::reference::TransportWebsocket;

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Notification {
    pub metadata: MetaData,
    pub payload: NotificationPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub subscription: NotificationSubscription,
    pub event: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubscription {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
    pub cost: u64,
    pub condition: HashMap<String, String>,
    pub transport: TransportWebsocket,
    pub created_at: DateTime<FixedOffset>,
}
