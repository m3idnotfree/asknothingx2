use serde::Serialize;
use serde_json::Value;

use super::MetaData;

#[derive(Debug, Serialize)]
pub struct Keepalive {
    pub metadata: MetaData,
    /// empty object
    pub payload: Value,
}
