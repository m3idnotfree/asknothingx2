use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// sesion_id
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: Option<u64>,
    pub reconnect_url: Option<String>,
    pub connected_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionPayload {
    pub session: Session,
}
