use asknothingx2_eventsub::twitch::reference::{Transport, TransportMethod};

#[test]
fn test_transport() {
    let test_transport =
        "{\"method\":\"websocket\",\n\"session_id\":\"AQoQexAWVYKSTIu4ec_2VAxyuhAB\"\n}";
    let transport = serde_json::from_str::<Transport>(test_transport);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.method, TransportMethod::Websocket);
    assert_eq!(
        transport.session_id,
        Some("AQoQexAWVYKSTIu4ec_2VAxyuhAB".to_string())
    );
}
