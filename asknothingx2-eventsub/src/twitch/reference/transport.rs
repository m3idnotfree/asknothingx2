use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transport {
    pub method: TransportMethod,
    /// webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    /// webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// websocket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// websocket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<String>,
    /// websocket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<String>,
}
