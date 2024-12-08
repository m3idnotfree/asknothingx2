use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Reconnect {
    pub metadata: MetaData,
    pub payload: ReconnectPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconnectPayload {
    pub session: ReconnectSession,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconnectSession {
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: Option<u64>,
    pub reconnect_url: String,
    pub connected_at: DateTime<FixedOffset>,
}
