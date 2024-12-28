use asknothingx2_eventsub::twitch::websocket_message::{Keepalive, MessageType};

fn_expected_payload!(
    payload: "{\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"session_keepalive\",\n        \"message_timestamp\": \"2023-07-19T10:11:12.634234626Z\"\n    },\n    \"payload\": {}\n}",
    from_str: Keepalive,
    block meta: {
        message_id: "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
        message_type: MessageType::SessionKeepalive,
        message_timestamp: "2023-07-19T10:11:12.634234626Z",
    },
    se contain: [
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"session_keepalive\"",
        "\"message_timestamp\":\"2023-07-19T10:11:12.634234626Z\"",
        "\"payload\":{}",
        ],
    se not: [ "\"subscription_type\"", "\"subscription_version\"" ]
);
