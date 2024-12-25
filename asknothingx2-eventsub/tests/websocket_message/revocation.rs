use asknothingx2_eventsub::twitch::{
    reference::{condition::Condition, transport::TransportMethod},
    subscription_types::types::SubscriptionTypes,
    websocket_message::{MessageType, Revocation},
};

#[test]
pub fn revocation() {
    let payload ="{\n\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"revocation\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"2\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"authorization_revoked\",\n            \"type\": \"channel.follow\",\n            \"version\": \"2\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        }\n    }\n}";
    let de = serde_json::from_str::<Revocation<Condition>>(payload);
    assert!(de.is_ok());

    let de = de.unwrap();
    let subscription_types = SubscriptionTypes::ChannelFollow;

    expected_de_metadata!(
        de,
        "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        MessageType::Revocation,
        "2022-11-16T10:11:12.464757833Z",
        Some(subscription_types.clone()),
        Some(subscription_types.version().to_string())
    );

    expect_de_subscription!(
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "authorization_revoked",
        kind = SubscriptionTypes::ChannelFollow,
        version = "2",
        cost = 1,
        created_at =
            chrono::DateTime::parse_from_rfc3339("2022-11-16T10:11:12.464757833Z").unwrap()
    );

    expect_de_transport!(
        de,
        TransportMethod::Websocket,
        None,
        None,
        Some("AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string()),
        None,
        None,
        None
    );

    expect_de_condition!(
        de,
        broadcaster_user_id = Some("12826".to_string()),
        moderator_user_id = None,
        broadcaster_id = None,
        user_id = None,
        reward_id = None,
        client_id = None
    );

    let se = serde_json::to_string(&de);

    assert!(se.is_ok());
    let se = se.unwrap();
    se_contains!(
        se,
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
    );
}
