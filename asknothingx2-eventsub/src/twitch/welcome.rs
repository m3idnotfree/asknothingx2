use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Welcome {
    pub metadata: MetaData,
    pub payload: WelcomePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomePayload {
    pub session: WelcomeSession,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeSession {
    /// sesion_id
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: u64,
    pub reconnect_url: Option<String>,
    pub connected_at: DateTime<FixedOffset>,
}
