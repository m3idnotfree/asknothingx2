use asknothingx2_eventsub::twitch::{
    reference::TransportMethod,
    websocket::{MessageType, Notification},
};

mod util;

#[test]
fn test_notification() {
    let test_notification = "{\n    \"metadata\": {\n        \"message_id\": \"befa7b53-d79d-478f-86b9-120f112b044e\",\n        \"message_type\": \"notification\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"1\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"enabled\",\n            \"type\": \"channel.follow\",\n            \"version\": \"1\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        },\n        \"event\": {\n            \"user_id\": \"1337\",\n            \"user_login\": \"awesome_user\",\n            \"user_name\": \"Awesome_User\",\n            \"broadcaster_user_id\": \"12826\",\n            \"broadcaster_user_login\": \"twitch\",\n            \"broadcaster_user_name\": \"Twitch\",\n            \"followed_at\": \"2023-07-15T18:16:11.17106713Z\"\n        }\n    }\n}";
    let deserialized = serde_json::from_str::<Notification>(test_notification);
    assert!(deserialized.is_ok());

    let deserialized = deserialized.unwrap();

    deserialized_metadata!(
        deserialized.metadata,
        "befa7b53-d79d-478f-86b9-120f112b044e",
        MessageType::Notification,
        "2022-11-16T10:11:12.464757833Z",
        "channel.follow".to_string(),
        "1".to_string()
    );

    deserialized_payload!(
        deserialized.payload,
        subscription.id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        subscription.status = "enabled",
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
        contain = event, {
            "user_id" = "1337",
            "user_login" = "awesome_user",
            "user_name" = "Awesome_User",
            "broadcaster_user_id" = "12826",
            "broadcaster_user_login" = "twitch",
            "broadcaster_user_name" = "Twitch",
            "followed_at" = "2023-07-15T18:16:11.17106713Z"
        };
        value = subscription.condition, {
            "broadcaster_user_id"="12826"
        }
    );

    let serialized = serde_json::to_string(&deserialized);

    assert!(serialized.is_ok());
    let serialized = serialized.unwrap();

    serialized_contains!(
        serialized,
        "\"message_id\":\"befa7b53-d79d-478f-86b9-120f112b044e\"",
        "\"message_type\":\"notification\"",
        "\"message_timestamp\":\"2022-11-16T10:11:12.464757833Z\"",
        "\"subscription_type\":\"channel.follow\"",
        "\"subscription_version\":\"1\"",
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"status\":\"enabled\"",
        "\"type\":\"channel.follow\"",
        "\"version\":\"1\"",
        "\"cost\":1",
        "\"broadcaster_user_id\":\"12826\"",
        "\"method\":\"websocket\"",
        "\"session_id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"",
        "\"created_at\":\"2022-11-16T10:11:12.464757833Z\"",
        "\"user_id\":\"1337\"",
        "\"user_login\":\"awesome_user\"",
        "\"user_name\":\"Awesome_User\"",
        "\"broadcaster_user_id\":\"12826\"",
        "\"broadcaster_user_login\":\"twitch\"",
        "\"broadcaster_user_name\":\"Twitch\"",
        "\"followed_at\":\"2023-07-15T18:16:11.17106713Z\""
    );
}
