use asknothingx2_eventsub::twitch::{
    subscription_types::channel_subscriptions::ChannelFollowPayload,
    websocket_message::{MessageType, Notification},
};

fn_expected_payload!(
    payload: "{\n    \"metadata\": {\n        \"message_id\": \"befa7b53-d79d-478f-86b9-120f112b044e\",\n        \"message_type\": \"notification\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"2\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"enabled\",\n            \"type\": \"channel.follow\",\n            \"version\": \"2\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        },\n        \"event\": {\n            \"user_id\": \"1337\",\n            \"user_login\": \"awesome_user\",\n            \"user_name\": \"Awesome_User\",\n            \"broadcaster_user_id\": \"12826\",\n            \"broadcaster_user_login\": \"twitch\",\n            \"broadcaster_user_name\": \"Twitch\",\n            \"followed_at\": \"2023-07-15T18:16:11.17106713Z\"\n        }\n    }\n}",
    from_str: Notification<ChannelFollowPayload>,
    prefix: payload,
    block meta: {
        message_id: "befa7b53-d79d-478f-86b9-120f112b044e",
        message_type: MessageType::Notification,
        message_timestamp: "2022-11-16T10:11:12.464757833Z",
        subscription_type: Some(SubscriptionType::ChannelFollow),
        subscription_version: Some(SubscriptionType::ChannelFollow.version().to_string())
    },
    block subscription: {
        id: "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status: "enabled",
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
    block event: {
        user_id: "1337",
        user_login: "awesome_user",
        user_name: "Awesome_User",
        broadcaster_user_id: "12826",
        broadcaster_user_login: "twitch",
        broadcaster_user_name: "Twitch",
        followed_at: "2023-07-15T18:16:11.17106713Z"
    },
    se contain: [
        "\"message_id\":\"befa7b53-d79d-478f-86b9-120f112b044e\"",
        "\"message_type\":\"notification\"",
        "\"message_timestamp\":\"2022-11-16T10:11:12.464757833Z\"",
        "\"subscription_type\":\"channel.follow\"",
        "\"subscription_version\":\"2\"",
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"status\":\"enabled\"",
        "\"type\":\"channel.follow\"",
        "\"version\":\"2\"",
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
        "\"broadcaster_user_name\":\"Twitch\"" // "\"followed_at\":\"2023-07-15T18:16:11.17106713Z\""
    ]
);
