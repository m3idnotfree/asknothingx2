use serde::{Deserialize, Serialize};

// If you need resiliency against replay attacks, consider the following:
//
// Make sure the value in the message_timestamp field isn’t older than 10 minutes.
// Make sure you haven’t seen the ID in the message_id field before.

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
    Conduit,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Transport {
//     pub method: TransportMethod,
//     /// webhook
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub callback: Option<String>,
//     /// webhook
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub secret: Option<String>,
//     /// websocket
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub session_id: Option<String>,
//     /// websocket
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub connected_at: Option<String>,
//     /// websocket
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub disconnected_at: Option<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "transport")]
pub struct TransportWh {
    pub method: TransportMethod,
    /// webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    /// webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "transport")]
pub struct TransportWs {
    pub method: TransportMethod,
    pub session_id: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub connected_at: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub disconnected_at: Option<String>,
}
