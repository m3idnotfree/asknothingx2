use asknothingx2_util::api::request::{RequestBody, RequestParts};
use bytes::Bytes;
use http::Method;
use serde_json::json;
use url::Url;

#[test]
fn test_zero_byte_handling() {
    let data_with_nulls = vec![0u8, 1u8, 0u8, 2u8, 0u8];
    let body = RequestBody::from_vec(data_with_nulls.clone());
    assert_eq!(body.content_length(), Some(5));
    assert!(!body.is_empty());
}

#[test]
fn test_special_json_values() {
    let special_json = json!({
        "null_value": null,
        "empty_string": "",
        "empty_array": [],
        "empty_object": {},
        "zero": 0,
        "negative": -1,
        "float":42.7385,
        "large_number": 1e20,
        "boolean_true": true,
        "boolean_false": false
    });

    let body = RequestBody::from_json(special_json);
    assert!(!body.is_empty());
    assert!(body.content_length().is_some());
}

#[test]
fn test_form_with_empty_values() {
    let form_data = vec![
        ("normal_field".to_string(), "normal_value".to_string()),
        ("empty_field".to_string(), "".to_string()),
        ("".to_string(), "empty_key".to_string()),
        ("".to_string(), "".to_string()),
    ];

    let body = RequestBody::from_form(form_data);
    assert!(!body.is_empty());
    assert!(body.content_length().is_some());
}

#[test]
fn test_request_parts_edge_cases() {
    let url = Url::parse("https://example.com").unwrap();

    let methods = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
        Method::CONNECT,
        Method::PATCH,
        Method::TRACE,
    ];

    for method in methods {
        let parts = RequestParts::new(method.clone(), url.clone());
        assert_eq!(parts.method, method);
    }
}

#[test]
fn test_header_edge_cases() {
    let url = Url::parse("https://example.com").unwrap();

    let mut parts = RequestParts::new(Method::GET, url);
    parts
        .header("Empty-Value", "")
        .header("Numeric-Value", "12345")
        .header("Special-Chars", "!@#$%^&*()")
        .header("Unicode-Header", "测试");

    assert!(parts.headers.len() >= 4);
}

#[test]
fn test_debug_formatting_edge_cases() {
    let edge_cases = vec![
        RequestBody::from_static(""),
        RequestBody::from_string("\n\r\t".to_string()),
        RequestBody::from_bytes(Bytes::from(vec![0u8; 1000])),
        RequestBody::from_json(
            json!({"very_long_key_name_that_might_cause_formatting_issues": "and_a_very_long_value_that_should_be_truncated_in_debug_output_because_it_exceeds_reasonable_length"}),
        ),
    ];

    for body in edge_cases {
        let debug_output = format!("{body:?}",);
        assert!(!debug_output.is_empty());
        assert!(debug_output.len() < 10000);
    }
}
