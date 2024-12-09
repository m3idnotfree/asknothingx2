use asknothingx2_eventsub::twitch::ws::{MessageType, Welcome};

mod util;

#[test]
fn test_welcome() {
    let test_welecome ="{\n  \"metadata\": {\n    \"message_id\": \"96a3f3b5-5dec-4eed-908e-e11ee657416c\",\n    \"message_type\": \"session_welcome\",\n    \"message_timestamp\": \"2023-07-19T14:56:51.634234626Z\"\n  },\n  \"payload\": {\n    \"session\": {\n      \"id\": \"AQoQILE98gtqShGmLD7AM6yJThAB\",\n      \"status\": \"connected\",\n      \"connected_at\": \"2023-07-19T14:56:51.616329898Z\",\n      \"keepalive_timeout_seconds\": 10,\n      \"reconnect_url\": null\n    }\n  }\n}";
    let deserialized = serde_json::from_str::<Welcome>(test_welecome);
    assert!(deserialized.is_ok());

    let deserialized = deserialized.unwrap();
    deserialized_metadata!(
        deserialized.metadata,
        "96a3f3b5-5dec-4eed-908e-e11ee657416c",
        MessageType::SessionWelcome,
        "2023-07-19T14:56:51.634234626Z"
    );

    deserialized_payload!(
        deserialized.payload,
        session.id = "AQoQILE98gtqShGmLD7AM6yJThAB".to_string(),
        session.status = "connected".to_string(),
        session.keepalive_timeout_seconds = 10,
        session.reconnect_url = None;
        time = (session.connected_at = "2023-07-19T14:56:51.616329898Z")
    );

    let serialized = serde_json::to_string(&deserialized);

    assert!(serialized.is_ok());
    let serialized = serialized.unwrap();

    serialized_contains!(
        serialized,
        "\"message_id\":\"96a3f3b5-5dec-4eed-908e-e11ee657416c\"",
        "\"message_type\":\"session_welcome\"",
        "\"message_timestamp\":\"2023-07-19T14:56:51.634234626Z\"",
        "\"id\":\"AQoQILE98gtqShGmLD7AM6yJThAB\"",
        "\"status\":\"connected\"",
        "\"keepalive_timeout_seconds\":10",
        "\"reconnect_url\":null",
        "\"connected_at\":\"2023-07-19T14:56:51.616329898Z\"",
        not = ("\"subscription_type\"", "\"subscription_version\"")
    );
}
