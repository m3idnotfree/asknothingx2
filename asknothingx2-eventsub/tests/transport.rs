use asknothingx2_eventsub::twitch::reference::{TransportMethod, TransportWh, TransportWs};

#[test]
fn test_transport_ws() {
    let test_transport =
        "{\"method\":\"websocket\",\n\"session_id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n}";
    let transport = serde_json::from_str::<TransportWs>(test_transport);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.method, TransportMethod::Websocket);
    assert_eq!(
        transport.session_id,
        "AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string()
    );
}

#[test]
fn test_transport_wh() {
    let test_transport =
        "{\"method\": \"webhook\",\n      \"callback\": \"https://example.com/webhooks/callback\"\n}";
    let transport = serde_json::from_str::<TransportWh>(test_transport);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.method, TransportMethod::Webhook);
    assert_eq!(
        transport.callback,
        Some("https://example.com/webhooks/callback".to_string())
    );
}
