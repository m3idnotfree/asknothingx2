use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::{
    ConduitShardDisabledPayload, ConduitShardDisabledRequest,
};

fn_expected_request!(
    request: ConduitShardDisabledRequest::webhook(
        "uo6dggojyb8d6soh92zknwmi5ej1q2",
        "https://example.com/webhooks/callback",
        Some("s3cRe7"),
    ),
    body: {
        contain: [
            "\"type\":\"conduit.shard.disabled\"",
            "\"version\":\"1\"",
            "\"client_id\":\"uo6dggojyb8d6soh92zknwmi5ej1q2\"",
            "\"method\":\"webhook\"",
            "\"callback\":\"https://example.com/webhooks/callback\"",
            "\"secret\":\"s3cRe7\""
        ]
    }
);

fn_expected_payload!(
    payload: "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"conduit.shard.disabled\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"client_id\": \"uo6dggojyb8d6soh92zknwmi5ej1q2\"\n        },\n        \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2023-04-11T10:11:12.123Z\"\n    },\n    \"event\": {\n        \"conduit_id\": \"bfcfc993-26b1-b876-44d9-afe75a379dac\",\n        \"shard_id\": \"4\",\n        \"status\": \"websocket_disconnected\",\n        \"transport\": {\n            \"method\": \"websocket\",\n            \"session_id\": \"ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9\",\n            \"connected_at\": \"2020-11-10T14:32:18.730260295Z\",\n            \"disconnected_at\": \"2020-11-11T14:32:18.730260295Z\"\n        }\n    }\n}",
    from_str: ConduitShardDisabledPayload,
    block subscription: {
        id : "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status : "enabled",
        kind : SubscriptionType::ConduitShardDisabled,
        version : "1",
        cost : 0,
        created_at : "2023-04-11T10:11:12.123Z"
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
    block condition: {
        client_id: "uo6dggojyb8d6soh92zknwmi5ej1q2",
        conduit_id: None
    },
    block event: {
        conduit_id: "bfcfc993-26b1-b876-44d9-afe75a379dac",
        shard_id: "4",
        status: "websocket_disconnected"
    },
    extra event.transport: {
        method: TransportMethod::Websocket,
        session_id: Some("ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9".to_string()),
        connected_at: Some(chrono::DateTime::parse_from_rfc3339("2020-11-10T14:32:18.730260295Z").unwrap()),
        disconnected_at: Some(chrono::DateTime::parse_from_rfc3339("2020-11-11T14:32:18.730260295Z").unwrap())
    },
    se contain: [
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"type\":\"conduit.shard.disabled\"",
        "\"version\":\"1\"",
        "\"status\":\"enabled\"",
        "\"cost\":0",
        "\"client_id\":\"uo6dggojyb8d6soh92zknwmi5ej1q2\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"created_at\":\"2023-04-11T10:11:12.123Z\"",
        "\"conduit_id\":\"bfcfc993-26b1-b876-44d9-afe75a379dac\"",
        "\"shard_id\":\"4\"",
        "\"status\":\"websocket_disconnected\"",
        "\"method\":\"websocket\"",
        "\"session_id\":\"ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9\"",
        "\"connected_at\":\"2020-11-10T14:32:18.730260295Z\"",
        "\"disconnected_at\":\"2020-11-11T14:32:18.730260295Z\""
    ]
);
