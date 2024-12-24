use serde::{Deserialize, Serialize};

use crate::twitch::{payload::SubscriptionEventPayload, websocket_message::MessageType};

use super::MetaData;

/// https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#notification-message
#[derive(Debug, Serialize)]
pub struct Notification<Condition, Event> {
    pub metadata: MetaData,
    pub payload: SubscriptionEventPayload<Condition, Event>,
}

impl<'de, Condition: Deserialize<'de>, Event: Deserialize<'de>> Deserialize<'de>
    for Notification<Condition, Event>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::{
            de::{Unexpected, Visitor},
            Deserialize,
        };

        struct NotificationVisitor<Condition, Event>(std::marker::PhantomData<(Condition, Event)>);

        impl<'de, Condition: Deserialize<'de>, Event: Deserialize<'de>> Visitor<'de>
            for NotificationVisitor<Condition, Event>
        {
            type Value = Notification<Condition, Event>;

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
