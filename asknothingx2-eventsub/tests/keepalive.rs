use asknothingx2_eventsub::twitch::websocket::{Keepalive, MessageType};

mod util;

#[test]
fn test_keepalive() {
    let test_keepalive ="{\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"session_keepalive\",\n        \"message_timestamp\": \"2023-07-19T10:11:12.634234626Z\"\n    },\n    \"payload\": {}\n}";

    let deserialized = serde_json::from_str::<Keepalive>(test_keepalive);
    assert!(deserialized.is_ok());
    let deserialized = deserialized.unwrap();
    deserialized_metadata!(
        deserialized.metadata,
        "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        MessageType::SessionKeepalive,
        "2023-07-19T10:11:12.634234626Z"
    );

    let serialized = serde_json::to_string(&deserialized);
    assert!(serialized.is_ok());
    let serialized = serialized.unwrap();

    serialized_contains!(
        serialized,
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"session_keepalive\"",
        "\"message_timestamp\":\"2023-07-19T10:11:12.634234626Z\"",
        "\"payload\":{}",
        not = ("\"subscription_type\"", "\"subscription_version\"")
    );
}

// > git commit message
//
// Based on the test file content, here's an appropriate commit message:
//
// ```
// test(eventsub): add keepalive message test
//
// Add unit test for Twitch EventSub keepalive message serialization
// and deserialization, verifying message structure and metadata fields.
// ```
//
// This follows conventional commit format with:
// - Type: test (adding or modifying tests)
// - Scope: eventsub (the EventSub functionality)
// - Brief description of what was added
// - Additional context in the body explaining what the test verifies
//
