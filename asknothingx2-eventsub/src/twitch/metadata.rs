use std::fmt::Display;

use chrono::{DateTime, FixedOffset};
use serde::{de::Expected, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub message_id: String,
    pub message_type: MessageType,
    pub message_timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
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
