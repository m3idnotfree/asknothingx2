//! https://dev.twitch.tv/docs/eventsub/handling-websocket-events
//! NOTE All timestamps are in RFC3339 format and use nanoseconds instead of milliseconds.
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod metadata;

pub use metadata::{MessageType, MetaData};

use crate::twitch::subscription::SubscriptionPayload;

macro_rules! twitch_websocket_message {
    (
    $(#[$meta:meta])*
    $name:ident,
    $(#[$field_meta:meta])*
    $payload:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name {
            pub metadata: MetaData,
            $(#[$field_meta])*
            pub payload: $payload,
        }
    };
    (
    $(#[$meta:meta])*
    $name:ident<$generic:ident>
    $(#[$field_meta:meta])*
    ) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name<$generic> {
            pub metadata: MetaData,
            $(#[$field_meta])*
            pub payload: $generic,
        }
    };

    (
    $(#[$meta:meta])*
    $name:ident<$generic:ident>,
    $(#[$field_meta:meta])*
    $payload:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug, serde::Serialize)]
        pub struct $name<$generic> {
            pub metadata: MetaData,
            $(#[$field_meta])*
            pub payload: $payload<$generic>,
        }
    };
}

twitch_websocket_message!(
    /// IMPORTANT
    /// By default, you have 10 seconds from the time you receive the Welcome message to subscribe to an event,
    /// unless otherwise specified when connecting.
    /// If you don’t subscribe within this timeframe,
    /// the server closes the connection.
    /// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#welcome-message
    Welcome,
    SessionPayload
);

twitch_websocket_message!(
    /// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#keepalive-message
    Keepalive,
    /// empty object
    Value
);
twitch_websocket_message!(
    /// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#reconnect-message
    Reconnect,
    SessionPayload
);
twitch_websocket_message!(
/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#revocation-message
    Revocation<Condition>,
    SubscriptionPayload
);

twitch_websocket_message!(
    /// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#notification-message
    Notification<T>
);

macro_rules! impl_deserialize {
    ($struct:ident, $name:literal, $payload:ident, $message_type:ident) => {
        impl<'de> Deserialize<'de> for $struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::{Error, Unexpected, Visitor};

                struct EventVisitor;

                impl<'de> Visitor<'de> for EventVisitor {
                    type Value = $struct;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($name)
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut metadata: Option<MetaData> = None;
                        let mut payload: Option<$payload> = None;

                        while let Some(key) = map.next_key::<&str>()? {
                            match key {
                                "metadata" => {
                                    if metadata.is_some() {
                                        return Err(Error::duplicate_field("metadata"));
                                    }
                                    metadata = Some(map.next_value()?);
                                }
                                "payload" => {
                                    if payload.is_some() {
                                        return Err(Error::duplicate_field("payload"));
                                    }
                                    payload = Some(map.next_value()?);
                                }
                                _ => {
                                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                                }
                            }
                        }

                        let metadata = metadata.ok_or_else(|| Error::missing_field("metadata"))?;
                        let payload = payload.ok_or_else(|| Error::missing_field("payload"))?;

                        if metadata.message_type != MessageType::$message_type {
                            return Err(Error::invalid_type(
                                Unexpected::Str(&metadata.message_type.to_string()),
                                &MessageType::$message_type,
                            ));
                        }

                        Ok($struct { metadata, payload })
                    }
                }

                deserializer.deserialize_map(EventVisitor)
            }
        }
    };
}

impl_deserialize!(Welcome, "Welcome", SessionPayload, SessionWelcome);
impl_deserialize!(Keepalive, "Keepalive", Value, SessionKeepalive);
impl_deserialize!(Reconnect, "Reconnect", SessionPayload, SessionReconnect);

macro_rules! impl_deserialize_websocket_message_with_subscriptionpayload {
    ($struct:ident, $name:literal, $payload:ident, $message_type:ident) => {
        impl<'de, T> serde::Deserialize<'de> for $struct<T>
        where
            T: serde::Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                struct _Visitor<T> {
                    _marker: std::marker::PhantomData<T>,
                }

                impl<'de, T> serde::de::Visitor<'de> for _Visitor<T>
                where
                    T: serde::Deserialize<'de>,
                {
                    type Value = $struct<T>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($name)
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut metadata: Option<MetaData> = None;
                        let mut payload: Option<$payload<T>> = None;

                        while let Some(key) = map.next_key::<&str>()? {
                            match key {
                                "metadata" => {
                                    if metadata.is_some() {
                                        return Err(Error::duplicate_field("metadata"));
                                    }
                                    metadata = Some(map.next_value()?);
                                }
                                "payload" => {
                                    if payload.is_some() {
                                        return Err(Error::duplicate_field("payload"));
                                    }
                                    payload = Some(map.next_value()?);
                                }
                                _ => {
                                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                                }
                            }
                        }

                        let metadata = metadata.ok_or_else(|| Error::missing_field("metadata"))?;
                        let payload = payload.ok_or_else(|| Error::missing_field("payload"))?;

                        if metadata.message_type != MessageType::$message_type {
                            return Err(Error::invalid_type(
                                serde::de::Unexpected::Str(&metadata.message_type.to_string()),
                                &MessageType::$message_type,
                            ));
                        }

                        Ok($struct { metadata, payload })
                    }
                }

                deserializer.deserialize_map(_Visitor {
                    _marker: std::marker::PhantomData,
                })
            }
        }
    };
}

impl_deserialize_websocket_message_with_subscriptionpayload!(
    Revocation,
    "Revocation",
    SubscriptionPayload,
    Revocation
);

impl<'de, Payload: Deserialize<'de>> Deserialize<'de> for Notification<Payload> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::{
            de::{Unexpected, Visitor},
            Deserialize,
        };
        struct NotificationVisitor<Payload>(std::marker::PhantomData<Payload>);

        impl<'de, Payload: Deserialize<'de>> Visitor<'de> for NotificationVisitor<Payload> {
            type Value = Notification<Payload>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Notification")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut metadata: Option<MetaData> = None;
                let mut payload = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "metadata" => {
                            if metadata.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        }
                        "payload" => {
                            if payload.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let metadata =
                    metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?;
                let payload = payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;

                if metadata.message_type != MessageType::Notification {
                    return Err(serde::de::Error::invalid_type(
                        Unexpected::Str(&metadata.message_type.to_string()),
                        &MessageType::Notification,
                    ));
                }

                Ok(Notification { metadata, payload })
            }
        }

        deserializer.deserialize_map(NotificationVisitor(std::marker::PhantomData))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionPayload {
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// sesion_id
    pub id: String,
    pub status: String,
    pub keepalive_timeout_seconds: Option<u64>,
    pub reconnect_url: Option<String>,
    pub connected_at: DateTime<FixedOffset>,
}
