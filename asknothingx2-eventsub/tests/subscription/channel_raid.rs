use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::{
    ChannelRaidPayload, ChannelRaidRequest,
};

fn_expected_request!(
    request: ChannelRaidRequest::webhook(
        "https://example.com/webhooks/callback",
        Some("s3cRe7")
        )
        .set_to_broadcacter_user_id("1337"),
    body: {
        contain: [
            "\"type\":\"channel.raid\"",
            "\"version\":\"1\"",
            "\"to_broadcaster_user_id\":\"1337\"",
            "\"method\":\"webhook\"",
            "\"callback\":\"https://example.com/webhooks/callback\"",
            "\"secret\":\"s3cRe7\""
        ]
    }
);

fn_expected_payload!(
    payload: "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"channel.raid\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"to_broadcaster_user_id\": \"1337\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"from_broadcaster_user_id\": \"1234\",\n        \"from_broadcaster_user_login\": \"cool_user\",\n        \"from_broadcaster_user_name\": \"Cool_User\",\n        \"to_broadcaster_user_id\": \"1337\",\n        \"to_broadcaster_user_login\": \"cooler_user\",\n        \"to_broadcaster_user_name\": \"Cooler_User\",\n        \"viewers\": 9001\n    }\n}",
    from_str: ChannelRaidPayload,
    block subscription: {
        id : "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status : "enabled",
        kind : SubscriptionType::ChannelRaid,
        version : "1",
        cost : 0,
        created_at :
            "2019-11-16T10:11:12.634234626Z"
    },
    block transport : {
        method: TransportMethod::Webhook,
        callback: Some("https://example.com/webhooks/callback".to_string()),
        secret: None,
        session_id: None,
        conduit_id: None,
        connected_at: None,
        disconnected_at: None
    },
    block condition : {
        from_broadcaster_user_id : None,
        to_broadcaster_user_id : Some("1337".to_string())
    },
    block event : {
        from_broadcaster_user_id : "1234",
        from_broadcaster_user_login : "cool_user",
        from_broadcaster_user_name : "Cool_User",
        to_broadcaster_user_id : "1337",
        to_broadcaster_user_login : "cooler_user",
        to_broadcaster_user_name : "Cooler_User",
        viewers : 9001
    },
    se contain: [
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"type\":\"channel.raid\"",
        "\"version\":\"1\"",
        "\"status\":\"enabled\"",
        "\"cost\":0",
        "\"to_broadcaster_user_id\":\"1337\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"created_at\":\"2019-11-16T10:11:12.634234626Z\"",
        "\"from_broadcaster_user_id\":\"1234\"",
        "\"from_broadcaster_user_login\":\"cool_user\"",
        "\"from_broadcaster_user_name\":\"Cool_User\"",
        "\"to_broadcaster_user_id\":\"1337\"",
        "\"to_broadcaster_user_login\":\"cooler_user\"",
        "\"to_broadcaster_user_name\":\"Cooler_User\"",
        "\"viewers\":9001"
    ]
);
