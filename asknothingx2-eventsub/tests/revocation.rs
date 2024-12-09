use asknothingx2_eventsub::twitch::{
    reference::TransportMethod,
    ws::{MessageType, Revocation},
};

mod util;

#[test]
fn test_revocation() {
    let test_revocation ="{\n\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"revocation\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"1\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"authorization_revoked\",\n            \"type\": \"channel.follow\",\n            \"version\": \"1\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        }\n    }\n}";
    let deserialized = serde_json::from_str::<Revocation>(test_revocation);
    assert!(deserialized.is_ok());

    let deserialized = deserialized.unwrap();

    deserialized_metadata!(
        deserialized.metadata,
        "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        MessageType::Revocation,
        "2022-11-16T10:11:12.464757833Z",
        "channel.follow".to_string(),
        "1".to_string(),
    );

    deserialized_payload!(
        deserialized.payload,
        subscription.id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        subscription.status = "authorization_revoked",
        subscription.kind = "channel.follow",
        subscription.version ="1",
        subscription.cost = 1,
        subscription.transport.method = TransportMethod::Websocket,
        // subscription.transport.callback = None,
        // subscription.transport.secret = None,
        subscription.transport.session_id = "AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string();
        // subscription.transport.connected_at = None,
        // subscription.transport.disconnected_at = None;
        time = (subscription.created_at = "2022-11-16T10:11:12.464757833Z");
        contain = subscription.condition, {
            "broadcaster_user_id"="12826"
        }
    );

    let serialized = serde_json::to_string(&deserialized);

    assert!(serialized.is_ok());
    let serialized = serialized.unwrap();
    serialized_contains!(
        serialized,
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"revocation\"",
        "\"message_timestamp\":\"2022-11-16T10:11:12.464757833Z\"",
        "\"subscription_type\":\"channel.follow\"",
        "\"subscription_version\":\"1\"",
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"status\":\"authorization_revoked\"",
        "\"type\":\"channel.follow\"",
        "\"version\":\"1\"",
        "\"cost\":1",
        "\"broadcaster_user_id\":\"12826\"",
        "\"method\":\"websocket\"",
        "\"session_id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"",
        "\"created_at\":\"2022-11-16T10:11:12.464757833Z\""
    );
}
