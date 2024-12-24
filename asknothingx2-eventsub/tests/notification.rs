use asknothingx2_eventsub::twitch::{
    reference::{condition::Condition, event::ChannelFollowEvent, transport::TransportMethod},
    subscription_type::SubscriptionTypes,
    websocket_message::{MessageType, Notification},
};

#[macro_use]
mod util;

#[test]
fn test_notification() {
    let test_notification = "{\n    \"metadata\": {\n        \"message_id\": \"befa7b53-d79d-478f-86b9-120f112b044e\",\n        \"message_type\": \"notification\",\n        \"message_timestamp\": \"2022-11-16T10:11:12.464757833Z\",\n        \"subscription_type\": \"channel.follow\",\n        \"subscription_version\": \"2\"\n    },\n    \"payload\": {\n        \"subscription\": {\n            \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n            \"status\": \"enabled\",\n            \"type\": \"channel.follow\",\n            \"version\": \"2\",\n            \"cost\": 1,\n            \"condition\": {\n                \"broadcaster_user_id\": \"12826\"\n            },\n            \"transport\": {\n                \"method\": \"websocket\",\n                \"session_id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n            },\n            \"created_at\": \"2022-11-16T10:11:12.464757833Z\"\n        },\n        \"event\": {\n            \"user_id\": \"1337\",\n            \"user_login\": \"awesome_user\",\n            \"user_name\": \"Awesome_User\",\n            \"broadcaster_user_id\": \"12826\",\n            \"broadcaster_user_login\": \"twitch\",\n            \"broadcaster_user_name\": \"Twitch\",\n            \"followed_at\": \"2023-07-15T18:16:11.17106713Z\"\n        }\n    }\n}";
    let de = serde_json::from_str::<Notification<Condition, ChannelFollowEvent>>(test_notification);
    assert!(de.is_ok());

    let de = de.unwrap();
    let subscription_types = SubscriptionTypes::ChannelFollow;

    expected_de_metadata!(
        de,
        "befa7b53-d79d-478f-86b9-120f112b044e",
        MessageType::Notification,
        "2022-11-16T10:11:12.464757833Z",
        Some(subscription_types.clone()),
        Some(subscription_types.version().to_string())
    );

    expect_de_subscription!(
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
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

    expect_de_event!(
        de,
        user_id = "1337",
        user_login = "awesome_user",
        user_name = "Awesome_User",
        broadcaster_user_id = "12826",
        broadcaster_user_login = "twitch",
        broadcaster_user_name = "Twitch",
        followed_at =
            chrono::DateTime::parse_from_rfc3339("2023-07-15T18:16:11.17106713Z").unwrap()
    );
    let se = serde_json::to_string(&de);

    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
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
    );
}
