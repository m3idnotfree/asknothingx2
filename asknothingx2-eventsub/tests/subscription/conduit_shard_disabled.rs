use asknothingx2_eventsub::twitch::{
    payload::SubscriptionEventPayload,
    reference::{
        condition::ConduitShardDisabledCondition,
        event::ConduitShardDisabledEvent,
        transport::{Transport, TransportMethod},
    },
    subscription_types::{
        request::{IntoSubscriptionRequest, SubscriptionRequest},
        types::SubscriptionTypes,
    },
};

#[test]
pub fn request() {
    let req = SubscriptionRequest::new(
        SubscriptionTypes::ConduitShardDisabled,
        ConduitShardDisabledCondition::new("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        Transport::webhook("https://example.com/webhooks/callback").set_secret("s3cRe7"),
    );

    let req = req.into_body();

    se_contains!(
        req,
        "\"type\":\"conduit.shard.disabled\"",
        "\"version\":\"1\"",
        "\"client_id\":\"uo6dggojyb8d6soh92zknwmi5ej1q2\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"secret\":\"s3cRe7\""
    );
}

#[test]
pub fn payload() {
    let payload = "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"conduit.shard.disabled\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"client_id\": \"uo6dggojyb8d6soh92zknwmi5ej1q2\"\n        },\n        \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2023-04-11T10:11:12.123Z\"\n    },\n    \"event\": {\n        \"conduit_id\": \"bfcfc993-26b1-b876-44d9-afe75a379dac\",\n        \"shard_id\": \"4\",\n        \"status\": \"websocket_disconnected\",\n        \"transport\": {\n            \"method\": \"websocket\",\n            \"session_id\": \"ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9\",\n            \"connected_at\": \"2020-11-10T14:32:18.730260295Z\",\n            \"disconnected_at\": \"2020-11-11T14:32:18.730260295Z\"\n        }\n    }\n}";

    let de = serde_json::from_str::<
        SubscriptionEventPayload<ConduitShardDisabledCondition, ConduitShardDisabledEvent>,
    >(payload);
    assert!(de.is_ok());
    let de = de.unwrap();

    expect_de_subscription!(
        req,
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
        kind = SubscriptionTypes::ConduitShardDisabled,
        version = "1",
        cost = 0,
        created_at = chrono::DateTime::parse_from_rfc3339("2023-04-11T10:11:12.123Z").unwrap()
    );

    expect_de_transport!(
        req,
        de,
        TransportMethod::Webhook,
        Some("https://example.com/webhooks/callback".to_string()),
        None,
        None,
        None,
        None,
        None
    );

    expect_de_condition!(
        req,
        de,
        client_id = "uo6dggojyb8d6soh92zknwmi5ej1q2",
        conduit_id = None
    );
    let mut tran = Transport::websocket("ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9");
    tran.connected_at =
        Some(chrono::DateTime::parse_from_rfc3339("2020-11-10T14:32:18.730260295Z").unwrap());
    tran.disconnected_at =
        Some(chrono::DateTime::parse_from_rfc3339("2020-11-11T14:32:18.730260295Z").unwrap());

    expect_de_event!(
        req,
        de,
        conduit_id = "bfcfc993-26b1-b876-44d9-afe75a379dac",
        shard_id = "4",
        status = "websocket_disconnected"
    );
    assert_eq!(de.event.transport.method, TransportMethod::Websocket);
    assert_eq!(
        de.event.transport.session_id,
        Some("ad1c9fc3-0d99-4eb7-8a04-8608e8ff9ec9".to_string())
    );
    assert_eq!(
        de.event.transport.connected_at,
        Some(chrono::DateTime::parse_from_rfc3339("2020-11-10T14:32:18.730260295Z").unwrap())
    );
    assert_eq!(
        de.event.transport.disconnected_at,
        Some(chrono::DateTime::parse_from_rfc3339("2020-11-11T14:32:18.730260295Z").unwrap())
    );

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
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
    );
}
