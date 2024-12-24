use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{de::Expected, Deserialize, Serialize};

use crate::twitch::subscription_type::SubscriptionTypes;

#[derive(Debug, Serialize)]
pub struct MetaData {
    pub message_id: String,
    pub message_type: MessageType,
    pub message_timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<SubscriptionTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            message_id: String,
            message_type: MessageType,
            message_timestamp: DateTime<FixedOffset>,
            subscription_type: Option<SubscriptionTypes>,
            subscription_version: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let subscription_type = helper.subscription_type.map(|kind| {
            match (kind, helper.subscription_version.as_deref()) {
                (SubscriptionTypes::AutomodMessageHold, Some("2")) => {
                    SubscriptionTypes::AutomodMessageHoldV2
                }
                (SubscriptionTypes::AutomodMessageUpdate, Some("2")) => {
                    SubscriptionTypes::AutomodMessageUpdateV2
                }
                (SubscriptionTypes::ChannelModerate, Some("2")) => {
                    SubscriptionTypes::ChannelModerateV2
                }
                (kind, _) => kind,
            }
        });

        Ok(MetaData {
            message_id: helper.message_id,
            message_type: helper.message_type,
            message_timestamp: helper.message_timestamp,
            subscription_type,
            subscription_version: helper.subscription_version,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    SessionWelcome,
    SessionKeepalive,
    Notification,
    SessionReconnect,
    Revocation,
}

impl MessageType {
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::SessionWelcome => "session_welcome",
            MessageType::SessionKeepalive => "session_keppalive",
            MessageType::Notification => "nofitication",
            MessageType::SessionReconnect => "session_reconnect",
            MessageType::Revocation => "revocation",
        }
    }
}

impl Expected for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MessageType::SessionWelcome => f.write_str("session_welcome"),
            MessageType::SessionKeepalive => f.write_str("session_keppalive"),
            MessageType::Notification => f.write_str("nofitication"),
            MessageType::SessionReconnect => f.write_str("session_reconnect"),
            MessageType::Revocation => f.write_str("revocation"),
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::SessionWelcome => f.write_str("session_welcome"),
            MessageType::SessionKeepalive => f.write_str("session_keppalive"),
            MessageType::Notification => f.write_str("nofitication"),
            MessageType::SessionReconnect => f.write_str("session_reconnect"),
            MessageType::Revocation => f.write_str("revocation"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::twitch::{subscription_type::SubscriptionTypes, websocket_message::MessageType};

    use super::MetaData;

    #[test]
    fn deserialize_metadata() {
        let de: MetaData = serde_json::from_str("{\n        \"message_id\": \"befa7b53-d79d-478f-86b9-120f112b044e\",\n        \"message_type\": \"notification\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"automod.message.hold\",\n        \"subscription_version\": \"2\"\n    }").unwrap();
        assert_eq!(de.message_id, "befa7b53-d79d-478f-86b9-120f112b044e");
        assert_eq!(de.message_type, MessageType::Notification);
        assert_eq!(
            de.subscription_type,
            Some(SubscriptionTypes::AutomodMessageHoldV2)
        );
        assert_eq!(de.subscription_version, Some("2".to_string()));
    }
}
