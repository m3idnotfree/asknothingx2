use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketEventSub {
    Welcome(Welcome),
    Keepalive(Keepalive),
    Notification(Notification),
    Reconnect(Reconnect),
    Revocation(Revocation),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    pub metadata: MetaData,
    pub payload: WelcomePayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomePayload {
    pub session: WelcomeSession,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeSession {
    /// sesion_id
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: u64,
    pub reconnect_url: Option<String>,
    pub connected_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keepalive {
    pub metadata: MetaData,
    pub payload: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub metadata: MetaData,
    pub payload: NotificationPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub subscription: NotificationSubscription,
    pub event: HashMap<String, String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSubscription {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Reconnect {
    pub metadata: MetaData,
    pub payload: ReconnectPayload,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReconnectPayload {
    pub session: ReconnectSession,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReconnectSession {
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: Option<u64>,
    pub reconnect_url: String,
    pub connected_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub message_id: String,
    pub message_type: MessageType,
    pub message_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    SessionWelcome,
    SessionKeepalive,
    Notification,
    SessionReconnect,
    Revocation,
}
