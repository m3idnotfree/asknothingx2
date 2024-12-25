use asknothingx2_eventsub::twitch::{
    payload::SubscriptionEventPayload,
    reference::{
        condition::ExtensionBitsTransactionCreateCondition,
        event::ExtensionBitsTransactionEvent,
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
        SubscriptionTypes::ExtensionBitsTransactionCreate,
        ExtensionBitsTransactionCreateCondition::new("deadbeef".to_string()),
        Transport::webhook("https://example.com/webhooks/callback").set_secret("s3cRe7"),
    );

    let req = req.into_body();

    se_contains!(
        req,
        "\"type\":\"extension.bits_transaction.create\"",
        "\"version\":\"1\"",
        "\"extension_client_id\":\"deadbeef\"",
        "\"method\":\"webhook\"",
        "\"callback\":\"https://example.com/webhooks/callback\"",
        "\"secret\":\"s3cRe7\""
    );
}

#[test]
pub fn payload() {
    let payload = "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"extension.bits_transaction.create\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"extension_client_id\": \"deadbeef\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"id\": \"bits-tx-id\",\n        \"extension_client_id\": \"deadbeef\",\n        \"broadcaster_user_id\": \"1337\",\n        \"broadcaster_user_login\": \"cool_user\",\n        \"broadcaster_user_name\": \"Cool_User\",\n        \"user_name\": \"Coolest_User\",\n        \"user_login\": \"coolest_user\",\n        \"user_id\": \"1236\",\n        \"product\": {\n            \"name\": \"great_product\",\n            \"sku\": \"skuskusku\",\n            \"bits\": 1234,\n            \"in_development\": false\n        }\n    }\n}";

    let de = serde_json::from_str::<
        SubscriptionEventPayload<
            ExtensionBitsTransactionCreateCondition,
            ExtensionBitsTransactionEvent,
        >,
    >(payload);
    assert!(de.is_ok());
    let de = de.unwrap();

    expect_de_subscription!(
        req,
        de,
        id = "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status = "enabled",
        kind = SubscriptionTypes::ExtensionBitsTransactionCreate,
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

    expect_de_condition!(req, de, extension_client_id = "deadbeef".to_string());

    expect_de_event!(
        req,
        de,
        id = "bits-tx-id",
        extension_client_id = "deadbeef",
        broadcaster_user_id = "1337",
        broadcaster_user_login = "cool_user",
        broadcaster_user_name = "Cool_User",
        user_name = "Coolest_User",
        user_login = "coolest_user",
        user_id = "1236"
    );
    assert_eq!(de.event.product.name, "great_product");
    assert_eq!(de.event.product.sku, "skuskusku");
    assert_eq!(de.event.product.bits, 1234);
    assert!(!de.event.product.in_development);

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
        "\"id\":\"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\"",
        "\"type\":\"extension.bits_transaction.create\"",
        "\"version\":\"1\"",
        "\"status\":\"enabled\"",
        "\"cost\":0",
        "\"id\":\"bits-tx-id\"",
        "\"extension_client_id\":\"deadbeef\"",
        "\"broadcaster_user_id\":\"1337\"",
        "\"broadcaster_user_login\":\"cool_user\"",
        "\"broadcaster_user_name\":\"Cool_User\"",
        "\"user_name\":\"Coolest_User\"",
        "\"user_login\":\"coolest_user\"",
        "\"name\":\"great_product\"",
        "\"sku\":\"skuskusku\"",
        "\"bits\":1234",
        "\"in_development\":false"
    );
}
