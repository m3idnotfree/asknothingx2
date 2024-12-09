use serde::Serialize;
use serde_json::Value;

use super::MetaDataWithoutSub;

#[derive(Debug, Serialize)]
pub struct Keepalive {
    pub metadata: MetaDataWithoutSub,
    /// empty object
    pub payload: Value,
}
