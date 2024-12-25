use asknothingx2_eventsub::twitch::{
    payload::SubscriptionEventPayload,
    reference::{
        condition::Condition,
        event::ChannelFollowEvent,
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
        SubscriptionTypes::ChannelFollow,
        Condition::default()
            .set_broadcaster_user_id("1337")
            .set_moderator_user_id("1337"),
        Transport::webhook("https://example.com/webhooks/callback").set_secret("s3cRe7"),
    )
    .set_require(["moderator:read:followers"]);

    let req = req.into_body();

    se_contains!(
        req,
        "\"type\":\"channel.follow\"",
        "\"version\":\"2\"",
        "\"broadcaster_user_id\":\"1337\"",
        "\"moderator_user_id\":\"1337\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"secret\":\"s3cRe7\""
    );
}

#[test]
pub fn payload() {
    let payload = "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"channel.follow\",\n        \"version\": \"2\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n           \"broadcaster_user_id\": \"1337\",\n           \"moderator_user_id\": \"1337\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"user_id\": \"1234\",\n        \"user_login\": \"cool_user\",\n        \"user_name\": \"Cool_User\",\n        \"broadcaster_user_id\": \"1337\",\n        \"broadcaster_user_login\": \"cooler_user\",\n        \"broadcaster_user_name\": \"Cooler_User\",\n        \"followed_at\": \"2020-07-15T18:16:11.17106713Z\"\n    }\n}";
    let de =
        serde_json::from_str::<SubscriptionEventPayload<Condition, ChannelFollowEvent>>(payload);
    assert!(de.is_ok());
    let de = de.unwrap();

    expect_de_subscription!(
        req,
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
        kind = SubscriptionTypes::ChannelFollow,
        version = "2",
        cost = 0,
        created_at =
            chrono::DateTime::parse_from_rfc3339("2019-11-16T10:11:12.634234626Z").unwrap()
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
        broadcaster_user_id = Some("1337".to_string()),
        moderator_user_id = Some("1337".to_string()),
        broadcaster_id = None,
        user_id = None,
        reward_id = None,
        client_id = None
    );

    expect_de_event!(
        req,
        de,
        user_id = "1234",
        user_login = "cool_user",
        user_name = "Cool_User",
        broadcaster_user_id = "1337",
        broadcaster_user_login = "cooler_user",
        broadcaster_user_name = "Cooler_User",
        followed_at =
            chrono::DateTime::parse_from_rfc3339("2020-07-15T18:16:11.17106713Z").unwrap()
    );

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
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
    );
}
