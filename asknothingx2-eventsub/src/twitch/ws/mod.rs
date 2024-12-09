mod welcome;
pub use welcome::*;
mod metadata;
pub use metadata::*;
mod keepalive;
pub use keepalive::*;
mod notification;
pub use notification::*;
mod reconnect;
pub use reconnect::*;
mod revocation;
pub use revocation::*;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;

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

macro_rules! impl_deserialize {
    ($struct:ident, $name:literal, $payload:ident, $message_type:ident, $meta:ident) => {
        impl<'de> Deserialize<'de> for $struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum Field {
                    metadata,
                    payload,
                    ignore,
                }
                struct FieldVisiter;
                impl<'de> Visitor<'de> for FieldVisiter {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "metadata" => Ok(Field::metadata),
                            "payload" => Ok(Field::payload),
                            _ => Ok(Field::ignore),
                        }
                    }
                }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_identifier(FieldVisiter)
                    }
                }

                struct _Visiter;
                impl<'de> Visitor<'de> for _Visiter {
                    type Value = $struct;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("struct $name")
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        // let mut metadata: Option<MetaData> = None;
                        let mut metadata: Option<$meta> = None;
                        let mut payload: Option<$payload> = None;

                        while let Some(key) = map.next_key::<Field>()? {
                            match key {
                                Field::metadata => {
                                    if metadata.is_some() {
                                        return Err(serde::de::Error::duplicate_field("metadata"));
                                    }
                                    metadata = Some(map.next_value()?);
                                }
                                Field::payload => {
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
                        let payload =
                            payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;

                        if metadata.message_type != MessageType::$message_type {
                            return Err(serde::de::Error::invalid_type(
                                Unexpected::Str(&metadata.message_type.to_string()),
                                &MessageType::$message_type,
                            ));
                        }

                        Ok($struct { metadata, payload })
                    }
                }
                const FIELDS: &'static [&'static str] = &["metadata", "payload"];
                deserializer.deserialize_struct($name, FIELDS, _Visiter)
            }
        }
    };
}

impl_deserialize!(
    Welcome,
    "Welcome",
    WelcomePayload,
    SessionWelcome,
    MetaDataWithoutSub
);
impl_deserialize!(
    Keepalive,
    "Keepalive",
    Value,
    SessionKeepalive,
    MetaDataWithoutSub
);
impl_deserialize!(
    Notification,
    "Notification",
    NotificationPayload,
    Notification,
    MetaData
);
impl_deserialize!(
    Reconnect,
    "Reconnect",
    ReconnectPayload,
    SessionReconnect,
    MetaDataWithoutSub
);
impl_deserialize!(
    Revocation,
    "Revocation",
    RevocationPayload,
    Revocation,
    MetaData
);
