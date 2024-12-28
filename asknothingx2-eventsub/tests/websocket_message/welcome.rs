use asknothingx2_eventsub::twitch::websocket_message::{MessageType, Welcome};

fn_expected_payload!(
    payload: "{\n  \"metadata\": {\n    \"message_id\": \"96a3f3b5-5dec-4eed-908e-e11ee657416c\",\n    \"message_type\": \"session_welcome\",\n    \"message_timestamp\": \"2023-07-19T14:56:51.634234626Z\"\n  },\n  \"payload\": {\n    \"session\": {\n      \"id\": \"AQoQILE98gtqShGmLD7AM6yJThAB\",\n      \"status\": \"connected\",\n      \"connected_at\": \"2023-07-19T14:56:51.616329898Z\",\n      \"keepalive_timeout_seconds\": 10,\n      \"reconnect_url\": null\n    }\n  }\n}",
    from_str: Welcome,
    block meta: {
    message_id: "96a3f3b5-5dec-4eed-908e-e11ee657416c",
    message_type: MessageType::SessionWelcome,
    message_timestamp: "2023-07-19T14:56:51.634234626Z"
    },
    block session: {
        id: "AQoQILE98gtqShGmLD7AM6yJThAB".to_string(),
        status: "connected".to_string(),
        keepalive_timeout_seconds: Some(10),
        reconnect_url: None,
        connected_at: "2023-07-19T14:56:51.616329898Z"
    },
    se contain: [
    "\"message_id\":\"96a3f3b5-5dec-4eed-908e-e11ee657416c\"",
    "\"message_type\":\"session_welcome\"",
    "\"message_timestamp\":\"2023-07-19T14:56:51.634234626Z\"",
    "\"id\":\"AQoQILE98gtqShGmLD7AM6yJThAB\"",
    "\"status\":\"connected\"",
    "\"keepalive_timeout_seconds\":10",
    "\"reconnect_url\":null",
    "\"connected_at\":\"2023-07-19T14:56:51.616329898Z\"",
    ],
    se not: [ "\"subscription_type\"", "\"subscription_version\"" ]
);
