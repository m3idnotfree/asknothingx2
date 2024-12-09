use asknothingx2_eventsub::twitch::websocket::{MessageType, Reconnect};

mod util;

#[test]
fn test_reconnect() {
    let test_reconnect = "{\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"session_reconnect\",\n        \"message_timestamp\": \"2022-11-18T09:10:11.634234626Z\"\n    },\n    \"payload\": {\n        \"session\": {\n           \"id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\",\n           \"status\": \"reconnecting\",\n           \"keepalive_timeout_seconds\": null,\n           \"reconnect_url\": \"wss://eventsub.wss.twitch.tv?...\",\n           \"connected_at\": \"2022-11-16T10:11:12.634234626Z\"\n        }\n    }\n}";
    let deserialized = serde_json::from_str::<Reconnect>(test_reconnect);

    assert!(deserialized.is_ok());

    let deserialized = deserialized.unwrap();
    deserialized_metadata!(
        deserialized.metadata,
        "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        MessageType::SessionReconnect,
        "2022-11-18T09:10:11.634234626Z"
    );

    deserialized_payload!(
        deserialized.payload,
        session.id = "AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string(),
        session.status = "reconnecting".to_string(),
        session.keepalive_timeout_seconds = None,
        session.reconnect_url = "wss://eventsub.wss.twitch.tv?...";
        time = (session.connected_at = "2022-11-16T10:11:12.634234626Z")
    );

    let serialized = serde_json::to_string(&deserialized);

    assert!(serialized.is_ok());
    let serialized = serialized.unwrap();

    serialized_contains!(
        serialized,
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"session_reconnect\"",
        "\"message_timestamp\":\"2022-11-18T09:10:11.634234626Z\"",
        "\"id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"",
        "\"status\":\"reconnecting\"",
        "\"keepalive_timeout_seconds\":null",
        "\"reconnect_url\":\"wss://eventsub.wss.twitch.tv?...\"",
        "\"connected_at\":\"2022-11-16T10:11:12.634234626Z\""
    );
}
