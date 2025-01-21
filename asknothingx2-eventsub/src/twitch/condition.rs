use serde::{Deserialize, Serialize};

pub trait IntoCondition: Serialize + Sized {
    fn serialize_to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conditions>
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcaster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl Condition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_broadcaster_user_id<T: Into<String>>(&mut self, broadcaster_user_id: T) {
        self.broadcaster_user_id = Some(broadcaster_user_id.into());
    }

    pub fn set_moderator_user_id<T: Into<String>>(&mut self, moderator_user_id: T) {
        self.moderator_user_id = Some(moderator_user_id.into());
    }

    pub fn set_broadcaster_id<T: Into<String>>(&mut self, broadcaster_id: T) {
        self.broadcaster_id = Some(broadcaster_id.into());
    }

    pub fn set_user_id<T: Into<String>>(&mut self, user_id: T) {
        self.user_id = Some(user_id.into());
    }

    pub fn set_reward_id<T: Into<String>>(&mut self, reward_id: T) {
        self.reward_id = Some(reward_id.into());
    }

    pub fn set_client_id<T: Into<String>>(&mut self, client_id: T) {
        self.client_id = Some(client_id.into());
    }
}

impl IntoCondition for Condition {}
