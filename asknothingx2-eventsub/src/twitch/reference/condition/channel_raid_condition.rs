use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChannelRaidCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_broadcaster_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_broadcaster_user_id: Option<String>,
}

impl ChannelRaidCondition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_from_broadcaster_user_id<T: Into<String>>(
        mut self,
        from_broadcaster_user_id: T,
    ) -> Self {
        self.from_broadcaster_user_id = Some(from_broadcaster_user_id.into());
        self
    }

    pub fn set_to_broadcacter_user_id<T: Into<String>>(
        mut self,
        to_broadcaster_user_id: T,
    ) -> Self {
        self.to_broadcaster_user_id = Some(to_broadcaster_user_id.into());
        self
    }
}
