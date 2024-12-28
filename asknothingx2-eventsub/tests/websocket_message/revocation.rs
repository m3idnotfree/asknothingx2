use asknothingx2_eventsub::twitch::{
    websocket_message::{MessageType, Revocation},
    Condition,
};

fn_expected_payload!(
    payload: "{\n\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"revocation\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"2\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"authorization_revoked\",\n            \"type\": \"channel.follow\",\n            \"version\": \"2\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        }\n    }\n}",
    from_str: Revocation<Condition>,
        block meta: {
        message_id: "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        message_type: MessageType::Revocation,
        message_timestamp: "2022-11-16T10:11:12.464757833Z",
        subscription_type: Some(SubscriptionType::ChannelFollow),
        subscription_version: Some(SubscriptionType::ChannelFollow.version().to_string())
    },
    block subscription: {
        id: "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status: "authorization_revoked",
        kind: SubscriptionType::ChannelFollow,
        version: "2",
        cost: 1,
        created_at: "2022-11-16T10:11:12.464757833Z"
    },
    block transport: {
        method: TransportMethod::Websocket,
        callback: None,
        secret: None,
        session_id: Some("AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string()),
        conduit_id: None,
        connected_at: None,
        disconnected_at: None
    },
    block condition: {
            broadcaster_user_id: Some("12826".to_string()),
            moderator_user_id: None,
            broadcaster_id: None,
            user_id: None,
            reward_id: None,
            client_id: None
    },
    se contain: [
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"revocation\"",
        "\"message_timestamp\":\"2022-11-16T10:11:12.464757833Z\"",
        "\"subscription_type\":\"channel.follow\"",
        "\"subscription_version\":\"2\"",
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"status\":\"authorization_revoked\"",
        "\"type\":\"channel.follow\"",
        "\"version\":\"2\"",
        "\"cost\":1",
        "\"broadcaster_user_id\":\"12826\"",
        "\"method\":\"websocket\"",
        "\"session_id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"",
        "\"created_at\":\"2022-11-16T10:11:12.464757833Z\""
    ]
);
