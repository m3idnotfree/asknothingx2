use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChannelRaidCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_broadcaster_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_broadcaster_user_id: Option<String>,
}
