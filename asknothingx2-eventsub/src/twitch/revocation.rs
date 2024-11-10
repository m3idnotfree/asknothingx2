use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Revocation {
    pub metadata: MetaData,
    pub payload: RevocationPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationPayload {
    pub subscription: RevocationSubscription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevocationSubscription {
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
