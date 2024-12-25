use asknothingx2_eventsub::twitch::{
    payload::SubscriptionEventPayload,
    reference::{
        condition::ChannelRaidCondition,
        event::ChannelRaidEvent,
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
        SubscriptionTypes::ChannelRaid,
        ChannelRaidCondition::new().set_to_broadcacter_user_id("1337"),
        Transport::webhook("https://example.com/webhooks/callback").set_secret("s3cRe7"),
    );

    let req = req.into_body();

    se_contains!(
        req,
        "\"type\":\"channel.raid\"",
        "\"version\":\"1\"",
        "\"to_broadcaster_user_id\":\"1337\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"secret\":\"s3cRe7\""
    );
}

#[test]
pub fn payload() {
    let payload ="{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"channel.raid\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"to_broadcaster_user_id\": \"1337\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"from_broadcaster_user_id\": \"1234\",\n        \"from_broadcaster_user_login\": \"cool_user\",\n        \"from_broadcaster_user_name\": \"Cool_User\",\n        \"to_broadcaster_user_id\": \"1337\",\n        \"to_broadcaster_user_login\": \"cooler_user\",\n        \"to_broadcaster_user_name\": \"Cooler_User\",\n        \"viewers\": 9001\n    }\n}";

    let de = serde_json::from_str::<SubscriptionEventPayload<ChannelRaidCondition, ChannelRaidEvent>>(
        payload,
    );
    assert!(de.is_ok());
    let de = de.unwrap();

    expect_de_subscription!(
        req,
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
        kind = SubscriptionTypes::ChannelRaid,
        version = "1",
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
        from_broadcaster_user_id = None,
        to_broadcaster_user_id = Some("1337".to_string())
    );

    expect_de_event!(
        req,
        de,
        from_broadcaster_user_id = "1234",
        from_broadcaster_user_login = "cool_user",
        from_broadcaster_user_name = "Cool_User",
        to_broadcaster_user_id = "1337",
        to_broadcaster_user_login = "cooler_user",
        to_broadcaster_user_name = "Cooler_User",
        viewers = 9001
    );

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
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
    );
}
