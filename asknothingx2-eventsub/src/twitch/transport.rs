use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
    Conduit,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transport {
    pub method: TransportMethod,
    /// The callback URL where the notifications are sent.
    /// The URL must use the HTTPS protocol and port 443.
    /// See Processing an event.
    /// <https://dev.twitch.tv/docs/eventsub/handling-webhook-events/#processing-an-event>
    ///
    /// Specify this field only if method is set to webhook.
    /// NOTE: Redirects are not followed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    /// The secret used to verify the signature.
    /// The secret must be an ASCII string that’s a minimum of 10 characters long
    /// and a maximum of 100 characters long.
    /// For information about how the secret is used,
    /// see Verifying the event message.
    /// <https://dev.twitch.tv/docs/eventsub/handling-webhook-events/#verifying-the-event-message>
    ///
    /// Specify this field only if method is set to webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// An ID that identifies the WebSocket to send notifications to.
    /// When you connect to EventSub using WebSockets,
    /// the server returns the ID in the Welcome message.
    /// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#welcome-message>
    ///
    /// Specify this field only if method is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// An ID that identifies the conduit to send notifications to.
    /// When you create a conduit, the server returns the conduit ID.
    ///
    /// Specify this field only if method is set to conduit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conduit_id: Option<String>,
    /// The UTC date and time that the WebSocket connection was established.
    ///
    /// This is a response-only field that
    /// Create EventSub Subscription and
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    ///
    /// Get EventSub Subscription returns
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    ///
    /// if the method field is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<DateTime<FixedOffset>>,
    /// The UTC date and time that the WebSocket connection was lost.
    ///
    /// This is a response-only field that
    /// Get EventSub Subscription returns
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    ///
    /// if the method field is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<DateTime<FixedOffset>>,
}

impl Transport {
    pub fn websocket<T: Into<String>>(session_id: T) -> Self {
        Self {
            method: TransportMethod::Websocket,
            callback: None,
            secret: None,
            session_id: Some(session_id.into()),
            conduit_id: None,
            connected_at: None,
            disconnected_at: None,
        }
    }

    pub fn webhook<T: Into<String>>(callback: T, secret: Option<T>) -> Self {
        Self {
            method: TransportMethod::Webhook,
            callback: Some(callback.into()),
            secret: secret.map(Into::into),
            session_id: None,
            conduit_id: None,
            connected_at: None,
            disconnected_at: None,
        }
    }
}
