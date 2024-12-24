use asknothingx2_eventsub::twitch::websocket_message::{MessageType, Welcome};

#[macro_use]
mod util;

#[test]
fn test_welcome() {
    let test_welecome ="{\n  \"metadata\": {\n    \"message_id\": \"96a3f3b5-5dec-4eed-908e-e11ee657416c\",\n    \"message_type\": \"session_welcome\",\n    \"message_timestamp\": \"2023-07-19T14:56:51.634234626Z\"\n  },\n  \"payload\": {\n    \"session\": {\n      \"id\": \"AQoQILE98gtqShGmLD7AM6yJThAB\",\n      \"status\": \"connected\",\n      \"connected_at\": \"2023-07-19T14:56:51.616329898Z\",\n      \"keepalive_timeout_seconds\": 10,\n      \"reconnect_url\": null\n    }\n  }\n}";
    let de = serde_json::from_str::<Welcome>(test_welecome);
    assert!(de.is_ok());

    let de = de.unwrap();
    expected_de_metadata!(
        de,
        "96a3f3b5-5dec-4eed-908e-e11ee657416c",
        MessageType::SessionWelcome,
        "2023-07-19T14:56:51.634234626Z"
    );

    expected_de_session!(
        de,
        id = "AQoQILE98gtqShGmLD7AM6yJThAB".to_string(),
        status = "connected".to_string(),
        keepalive_timeout_seconds = Some(10),
        reconnect_url = None,
        connected_at =
            chrono::DateTime::parse_from_rfc3339("2023-07-19T14:56:51.616329898Z").unwrap()
    );

    let se = serde_json::to_string(&de);

    assert!(se.is_ok());
    let se = se.unwrap();

    se_contains!(
        se,
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
