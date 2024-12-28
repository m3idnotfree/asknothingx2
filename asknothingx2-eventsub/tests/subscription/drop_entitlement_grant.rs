use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::drop_entitlement_grant::{
    DropEntitlementGrantPayloadNotification, DropEntitlementGrantRequest,
};

fn_expected_request!(
    request: DropEntitlementGrantRequest::webhook(
        "9001",
        "https://example.com/webhooks/callback",
        Some("s3cRe7"),
    )
    .set_category_id("9002")
    .set_campaign_id("9003"),
    body: {
        contain: [
            "\"type\":\"drop.entitlement.grant\"",
            "\"version\":\"1\"",
            "\"organization_id\":\"9001\"",
            "\"category_id\":\"9002\"",
            "\"campaign_id\":\"9003\"",
            "\"method\":\"webhook\"",
            "\"callback\":\"https://example.com/webhooks/callback\"",
            "\"secret\":\"s3cRe7\"",
            "\"is_batching_enabled\":\"true\""
        ]
    }
);

fn_expected_payload!(
    payload: "{\n    \"subscription\": {\n    \"cost\":0,    \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"drop.entitlement.grant\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"condition\": {\n           \"organization_id\": \"9001\",\n            \"category_id\": \"9002\", \n            \"campaign_id\": \"9003\"  \n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"events\": [\n        {\n            \"id\": \"bf7c8577-e3e3-4881-a78a-e9446641d45d\",\n            \"data\": {\n                \"organization_id\": \"9001\",\n                \"category_id\": \"9002\",\n                \"category_name\": \"Fortnite\",\n                \"campaign_id\": \"9003\",\n                \"user_id\": \"1234\",\n                \"user_name\": \"Cool_User\", \n                \"user_login\": \"cool_user\",\n                \"entitlement_id\": \"fb78259e-fb81-4d1b-8333-34a06ffc24c0\",\n                \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n                \"created_at\": \"2019-01-28T04:17:53.325Z\"\n            }\n        },\n        {\n            \"id\": \"bf7c8577-e3e3-4881-a78a-e9446641d45c\",\n            \"data\": {\n                \"organization_id\": \"9001\",\n                \"category_id\": \"9002\",\n                \"category_name\": \"Fortnite\",\n                \"campaign_id\": \"9003\",\n                \"user_id\": \"12345\",\n                \"user_name\": \"Cooler_User\",\n                \"user_login\": \"cooler_user\",\n                \"entitlement_id\": \"fb78259e-fb81-4d1b-8333-34a06ffc24c0\",\n                \"benefit_id\": \"74c52265-e214-48a6-91b9-23b6014e8041\",\n                \"created_at\": \"2019-01-28T04:17:53.325Z\"\n            }\n        }\n    ]\n}",
    from_str: DropEntitlementGrantPayloadNotification,
    block subscription: {
        id : "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status : "enabled",
        kind : SubscriptionType::DropEntitlementGrant,
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
    block condition: {
        organization_id : "9001".to_string(),
        category_id : Some("9002".to_string()),
        campaign_id : Some("9003".to_string())
    },
    se contain: [
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
    ]
);
