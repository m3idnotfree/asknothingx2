use asknothingx2_eventsub::twitch::websocket_message::{MessageType, Reconnect};

fn_expected_payload!(
    payload: "{\n    \"metadata\": {\n        \"message_id\": \"84c1e79a-2a4b-4c13-ba0b-4312293e9308\",\n        \"message_type\": \"session_reconnect\",\n        \"message_timestamp\": \"2022-11-18T09:10:11.634234626Z\"\n    },\n    \"payload\": {\n        \"session\": {\n           \"id\": \"AQoQexAWVYKSTIu4ec_2VAxyuhAB\",\n           \"status\": \"reconnecting\",\n           \"keepalive_timeout_seconds\": null,\n           \"reconnect_url\": \"wss://eventsub.wss.twitch.tv?...\",\n           \"connected_at\": \"2022-11-16T10:11:12.634234626Z\"\n        }\n    }\n}",
    from_str: Reconnect,
    block meta: {
      message_id: "84c1e79a-2a4b-4c13-ba0b-4312293e9308",
      message_type: MessageType::SessionReconnect,
      message_timestamp: "2022-11-18T09:10:11.634234626Z"
    },
    block session: {
            id: "AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string(),
            status: "reconnecting".to_string(),
            keepalive_timeout_seconds: None,
            reconnect_url: Some("wss://eventsub.wss.twitch.tv?...".to_string()),
            connected_at: "2022-11-16T10:11:12.634234626Z"
    },
    se contain: [
        "\"message_id\":\"84c1e79a-2a4b-4c13-ba0b-4312293e9308\"",
        "\"message_type\":\"session_reconnect\"",
        "\"message_timestamp\":\"2022-11-18T09:10:11.634234626Z\"",
        "\"id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"",
        "\"status\":\"reconnecting\"",
        "\"keepalive_timeout_seconds\":null",
        "\"reconnect_url\":\"wss://eventsub.wss.twitch.tv?...\"",
        "\"connected_at\":\"2022-11-16T10:11:12.634234626Z\""
    ]
);
