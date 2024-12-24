//! https://dev.twitch.tv/docs/eventsub/handling-websocket-events
//! NOTE All timestamps are in RFC3339 format and use nanoseconds instead of milliseconds.
mod welcome;
pub use welcome::Welcome;
mod metadata;
pub use metadata::{MessageType, MetaData};
mod keepalive;
pub use keepalive::Keepalive;
mod notification;
pub use notification::Notification;
mod reconnect;
pub use reconnect::Reconnect;
mod revocation;
pub use revocation::Revocation;

use crate::twitch::payload::{SessionPayload, SubscriptionPayload};

use serde::Deserialize;
use serde_json::Value;

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
