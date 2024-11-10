use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Notification {
    pub metadata: MetaData,
    pub payload: NotificationPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub subscription: NotificationSubscription,
    pub event: HashMap<String, String>,
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
    pub transport: HashMap<String, String>,
    pub created_at: String,
}
