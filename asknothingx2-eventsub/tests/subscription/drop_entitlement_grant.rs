use asknothingx2_eventsub::twitch::{
    payload::DropEntitlementGrantPayload,
    reference::transport::{Transport, TransportMethod},
    subscription_types::{
        request::{DropEntitlementGrantRequest, IntoSubscriptionRequest},
        types::SubscriptionTypes,
    },
};

#[test]
pub fn request() {
    let req = DropEntitlementGrantRequest::new(
        "9001",
        Transport::webhook("https://example.com/webhooks/callback").set_secret("s3cRe7"),
    )
    .set_category_id("9002")
    .set_campaign_id("9003");

    let req = req.into_body();

    se_contains!(
        req,
        "\"type\":\"drop.entitlement.grant\"",
        "\"version\":\"1\"",
        "\"organization_id\":\"9001\"",
        "\"category_id\":\"9002\"",
        "\"campaign_id\":\"9003\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"secret\":\"s3cRe7\"",
        "\"is_batching_enabled\":\"true\""
    );
}

#[test]
pub fn payload() {
    let payload = "{\n    \"subscription\": {\n    \"cost\":0,    \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"drop.entitlement.grant\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"condition\": {\n           \"organization_id\": \"9001\",\n            \"category_id\": \"9002\", \n            \"campaign_id\": \"9003\"  \n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"events\": [\n        {\n            \"id\": \"bf7c8577-e3e3-4881-a78a-e9446641d45d\",\n            \"data\": {\n                \"organization_id\": \"9001\",\n                \"category_id\": \"9002\",\n                \"category_name\": \"Fortnite\",\n                \"campaign_id\": \"9003\",\n                \"user_id\": \"1234\",\n                \"user_name\": \"Cool_User\", \n                \"user_login\": \"cool_user\",\n                \"entitlement_id\": \"fb78259e-fb81-4d1b-8333-34a06ffc24c0\",\n                \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n                \"created_at\": \"2019-01-28T04:17:53.325Z\"\n            }\n        },\n        {\n            \"id\": \"bf7c8577-e3e3-4881-a78a-e9446641d45c\",\n            \"data\": {\n                \"organization_id\": \"9001\",\n                \"category_id\": \"9002\",\n                \"category_name\": \"Fortnite\",\n                \"campaign_id\": \"9003\",\n                \"user_id\": \"12345\",\n                \"user_name\": \"Cooler_User\",\n                \"user_login\": \"cooler_user\",\n                \"entitlement_id\": \"fb78259e-fb81-4d1b-8333-34a06ffc24c0\",\n                \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n                \"created_at\": \"2019-01-28T04:17:53.325Z\"\n            }\n        }\n    ]\n}";

    let de = serde_json::from_str::<DropEntitlementGrantPayload>(payload);
    assert!(de.is_ok());
    let de = de.unwrap();

    expect_de_subscription!(
        req,
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
        kind = SubscriptionTypes::DropEntitlementGrant,
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
        organization_id = "9001".to_string(),
        category_id = Some("9002".to_string()),
        campaign_id = Some("9003".to_string())
    );

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"type\":\"drop.entitlement.grant\"",
        "\"version\":\"1\"",
        "\"status\":\"enabled\"",
        "\"cost\":0",
        "\"organization_id\":\"9001\"",
        "\"category_id\":\"9002\"",
        "\"campaign_id\":\"9003\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"created_at\":\"2019-11-16T10:11:12.634234626Z\"",
        "\"id\":\"bf7c8577-e3e3-4881-a78a-e9446641d45d\"",
        "\"organization_id\":\"9001\"",
        "\"category_id\":\"9002\"",
        "\"category_name\":\"Fortnite\"",
        "\"campaign_id\":\"9003\"",
        "\"user_id\":\"1234\"",
        "\"user_name\":\"Cool_User\"",
        "\"user_login\":\"cool_user\"",
        "\"entitlement_id\":\"fb78259e-fb81-4d1b-8333-34a06ffc24c0\"",
        "\"benefit_id\":\"74c52265-e214-48a6-91b9-23b6014e8041\"",
        "\"created_at\":\"2019-01-28T04:17:53.325Z\"",
        "\"id\":\"bf7c8577-e3e3-4881-a78a-e9446641d45c\",",
        "\"organization_id\":\"9001\"",
        "\"category_id\":\"9002\"",
        "\"category_name\":\"Fortnite\"",
        "\"campaign_id\":\"9003\"",
        "\"user_id\":\"12345\"",
        "\"user_name\":\"Cooler_User\"",
        "\"user_login\":\"cooler_user\"",
        "\"entitlement_id\":\"fb78259e-fb81-4d1b-8333-34a06ffc24c0\"",
        "\"benefit_id\":\"74c52265-e214-48a6-91b9-23b6014e8041\"",
        "\"created_at\":\"2019-01-28T04:17:53.325Z\""
    );
}
