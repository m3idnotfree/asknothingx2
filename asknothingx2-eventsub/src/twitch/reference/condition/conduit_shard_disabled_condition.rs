use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConduitShardDisabledCondition {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conduit_id: Option<String>,
}

impl ConduitShardDisabledCondition {
    pub fn new<T: Into<String>>(client_id: T) -> Self {
        Self {
            client_id: client_id.into(),
            conduit_id: None,
        }
    }
}
