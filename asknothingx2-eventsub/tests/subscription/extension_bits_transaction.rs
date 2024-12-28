use asknothingx2_eventsub::twitch::subscription_types::channel_subscriptions::{
    ExtensionBitsTransactionPayload, ExtensionBitsTransactionRequest,
};

fn_expected_request!(
    request: ExtensionBitsTransactionRequest::webhook(
        "deadbeef",
        "https://example.com/webhooks/callback",
        Some("s3cRe7"),
    ),
    body: {
        contain: [
            "\"type\":\"extension.bits_transaction.create\"",
            "\"version\":\"1\"",
            "\"extension_client_id\":\"deadbeef\"",
            "\"method\":\"webhook\"",
            "\"callback\":\"https://example.com/webhooks/callback\"",
            "\"secret\":\"s3cRe7\""
        ]
    }
);

fn_expected_payload!(
    payload: "{\n    \"subscription\": {\n        \"id\": \"f1c2a387-161a-49f9-a165-0f21d7a4e1c4\",\n        \"type\": \"extension.bits_transaction.create\",\n        \"version\": \"1\",\n        \"status\": \"enabled\",\n        \"cost\": 0,\n        \"condition\": {\n            \"extension_client_id\": \"deadbeef\"\n        },\n         \"transport\": {\n            \"method\": \"webhook\",\n            \"callback\": \"https://example.com/webhooks/callback\"\n        },\n        \"created_at\": \"2019-11-16T10:11:12.634234626Z\"\n    },\n    \"event\": {\n        \"id\": \"bits-tx-id\",\n        \"extension_client_id\": \"deadbeef\",\n        \"broadcaster_user_id\": \"1337\",\n        \"broadcaster_user_login\": \"cool_user\",\n        \"broadcaster_user_name\": \"Cool_User\",\n        \"user_name\": \"Coolest_User\",\n        \"user_login\": \"coolest_user\",\n        \"user_id\": \"1236\",\n        \"product\": {\n            \"name\": \"great_product\",\n            \"sku\": \"skuskusku\",\n            \"bits\": 1234,\n            \"in_development\": false\n        }\n    }\n}",
    from_str: ExtensionBitsTransactionPayload,
    block subscription: {
        id: "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
        status: "enabled",
        kind: SubscriptionType::ExtensionBitsTransactionCreate,
        version: "1",
        cost: 0,
        created_at: "2019-11-16T10:11:12.634234626Z"
    },
    block transport: {
        method: TransportMethod::Webhook,
        callback: Some("https://example.com/webhooks/callback".to_string()),
        secret: None,
        session_id: None,
        conduit_id: None,
        connected_at: None,
        disconnected_at: None
    },
    block condition: {
        extension_client_id: "deadbeef".to_string()
    },
    block event: {
        id: "bits-tx-id",
        extension_client_id: "deadbeef",
        broadcaster_user_id: "1337",
        broadcaster_user_login: "cool_user",
        broadcaster_user_name: "Cool_User",
        user_name: "Coolest_User",
        user_login: "coolest_user",
        user_id: "1236"
    },
    extra event.product: {
                name: "great_product",
                sku: "skuskusku",
                bits: 1234,
                in_development: false
    },
    se contain: [
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
    ]
);
