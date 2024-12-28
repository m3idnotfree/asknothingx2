use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::{
    ChannelFollowPayload, ChannelFollowRequest,
};

fn_expected_request!(
    request: ChannelFollowRequest::webhook(
        "1337",
        "1337",
        "https://example.com/webhooks/callback",
        Some("s3cRe7"),
    ),
    body: {
        contain: [
            "\"type\":\"channel.follow\"",
            "\"version\":\"2\"",
            "\"broadcaster_user_id\":\"1337\"",
            "\"moderator_user_id\":\"1337\"",
            "\"method\":\"webhook\"",
            "\"callback\":\"https://example.com/webhooks/callback\"",
            "\"secret\":\"s3cRe7\""
        ]
    }
);

fn_expected_payload!(
    payload: "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"channel.follow\",\n        \"version\": \"2\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n           \"broadcaster_user_id\": \"1337\",\n           \"moderator_user_id\": \"1337\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"user_id\": \"1234\",\n        \"user_login\": \"cool_user\",\n        \"user_name\": \"Cool_User\",\n        \"broadcaster_user_id\": \"1337\",\n        \"broadcaster_user_login\": \"cooler_user\",\n        \"broadcaster_user_name\": \"Cooler_User\",\n        \"followed_at\": \"2020-07-15T18:16:11.17106713Z\"\n    }\n}",
    from_str: ChannelFollowPayload,
    block subscription: {
        id: "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status: "enabled",
        kind: SubscriptionType::ChannelFollow,
        version: "2",
        cost: 0,
        created_at: "2019-11-16T10:11:12.634234626Z"
    },
    block transport: {
        method: TransportMethod::Webhook,
        callback: Some("https://example.com/webhooks/callback".to_string()),
        secret: None,
        session_id: None,
        conduit_id: None,
        connected_at: None,
        disconnected_at: None
    },
    block condition: {
        broadcaster_user_id: Some("1337".to_string()),
        moderator_user_id: Some("1337".to_string()),
        broadcaster_id: None,
        user_id: None,
        reward_id: None,
        client_id: None
    },
    block event: {
        user_id: "1234",
        user_login: "cool_user",
        user_name: "Cool_User",
        broadcaster_user_id: "1337",
        broadcaster_user_login: "cooler_user",
        broadcaster_user_name: "Cooler_User",
        followed_at: "2020-07-15T18:16:11.17106713Z"
    },
    se contain: [
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"status\":\"enabled\"",
        "\"type\":\"channel.follow\"",
        "\"version\":\"2\"",
        "\"cost\":0",
        "\"broadcaster_user_id\":\"1337\"",
        "\"method\":\"webhook\"",
        "\"created_at\":\"2019-11-16T10:11:12.634234626Z\"",
        "\"user_id\":\"1234\"",
        "\"user_login\":\"cool_user\"",
        "\"user_name\":\"Cool_User\"",
        "\"broadcaster_user_id\":\"1337\"",
        "\"broadcaster_user_login\":\"cooler_user\"",
        "\"broadcaster_user_name\":\"Cooler_User\"",
        "\"followed_at\":\"2020-07-15T18:16:11.171067130Z\""
    ]
);
