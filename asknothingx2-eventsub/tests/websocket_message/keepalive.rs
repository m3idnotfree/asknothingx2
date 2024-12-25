use asknothingx2_eventsub::twitch::websocket_message::{Keepalive, MessageType};

#[test]
pub fn keepalive() {
    let payload ="{\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"session_keepalive\",\n        \"message_timestamp\": \"2023-07-19T10:11:12.634234626Z\"\n    },\n    \"payload\": {}\n}";

    let de = serde_json::from_str::<Keepalive>(payload);
    assert!(de.is_ok());
    let de = de.unwrap();
    expected_de_metadata!(
        de,
        "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        MessageType::SessionKeepalive,
        "2023-07-19T10:11:12.634234626Z"
    );

    let se = serde_json::to_string(&de);
    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"session_keepalive\"",
        "\"message_timestamp\":\"2023-07-19T10:11:12.634234626Z\"",
        "\"payload\":{}",
        not = ("\"subscription_type\"", "\"subscription_version\"")
    );
}
