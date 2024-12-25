use asknothingx2_eventsub::twitch::{
    subscription_types::types::SubscriptionTypes,
    websocket_message::{MessageType, MetaData},
};

#[test]
pub fn deserialize_metadata() {
    let de: MetaData = serde_json::from_str("{\n        \"message_id\": \"befa7b53-d79d-478f-86b9-120f112b044e\",\n        \"message_type\": \"notification\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"automod.message.hold\",\n        \"subscription_version\": \"2\"\n    }").unwrap();
    assert_eq!(de.message_id, "befa7b53-d79d-478f-86b9-120f112b044e");
    assert_eq!(de.message_type, MessageType::Notification);
    assert_eq!(
        de.message_timestamp,
        chrono::DateTime::parse_from_rfc3339("2022-11-16T10:11:12.464757833Z").unwrap()
    );
    assert_eq!(
        de.subscription_type,
        Some(SubscriptionTypes::AutomodMessageHoldV2)
    );
    assert_eq!(de.subscription_version, Some("2".to_string()));
}
