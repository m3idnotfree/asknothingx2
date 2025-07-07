mod stream;

use asknothingx2_util::api::request::RequestBody;
use bytes::Bytes;
use reqwest::multipart;
use serde_json::json;

#[test]
fn test_from_static() {
    let body = RequestBody::from_static("hello world");
    assert!(matches!(body, RequestBody::Static("hello world")));
    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(11));
}

#[test]
fn test_from_static_empty() {
    let body = RequestBody::from_static("");
    assert!(body.is_empty());
    assert_eq!(body.content_length(), Some(0));
}

#[test]
fn test_from_string() {
    let body = RequestBody::from_string("test string");
    assert!(matches!(body, RequestBody::String(_)));
    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(11));
}

#[test]
fn test_from_string_empty() {
    let body = RequestBody::from_string("");
    assert!(body.is_empty());
    assert_eq!(body.content_length(), Some(0));
}

#[test]
fn test_from_string_into() {
    let body = RequestBody::from_string(String::from("converted"));
    assert!(matches!(body, RequestBody::String(_)));
}

#[test]
fn test_from_bytes() {
    let data = Bytes::from("byte data");
    let body = RequestBody::from_bytes(data.clone());
    assert!(matches!(body, RequestBody::Bytes(_)));
    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(9));
}

#[test]
fn test_from_bytes_empty() {
    let body = RequestBody::from_bytes(Bytes::new());
    assert!(body.is_empty());
    assert_eq!(body.content_length(), Some(0));
}

#[test]
fn test_from_vec() {
    let data = vec![1, 2, 3, 4, 5];
    let body = RequestBody::from_vec(data);
    assert!(matches!(body, RequestBody::Vec(_)));
    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(5));
}

#[test]
fn test_from_vec_empty() {
    let body = RequestBody::from_vec(vec![]);
    assert!(body.is_empty());
    assert_eq!(body.content_length(), Some(0));
}

#[test]
fn test_from_json() {
    let json_value = json!({"key": "value", "number": 42});
    let body = RequestBody::from_json(json_value);
    assert!(matches!(body, RequestBody::Json(_)));
    assert!(!body.is_empty());
    assert!(body.content_length().is_some());
}

#[test]
fn test_from_json_null() {
    let body = RequestBody::from_json(json!(null));
    assert!(body.is_empty());
}

#[test]
fn test_from_json_serializable() {
    #[derive(serde::Serialize)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    let test_data = TestStruct {
        name: "John".to_string(),
        age: 30,
    };

    let result = RequestBody::from_json_serializable(&test_data);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), RequestBody::Json(_)));
}

#[test]
fn test_from_form() {
    let form_data = vec![
        ("name".to_string(), "John".to_string()),
        ("email".to_string(), "john@example.com".to_string()),
    ];
    let body = RequestBody::from_form(form_data);
    assert!(matches!(body, RequestBody::Form(_)));
    assert!(!body.is_empty());
    assert!(body.content_length().is_some());
}

#[test]
fn test_from_form_empty() {
    let body = RequestBody::from_form(vec![]);
    assert!(body.is_empty());
}

#[test]
fn test_from_form_pairs() {
    let pairs = [("key1", "value1"), ("key2", "value2")];
    let body = RequestBody::from_form_pairs(pairs);
    assert!(matches!(body, RequestBody::Form(_)));
    assert!(!body.is_empty());
}

#[test]
fn test_from_multipart() {
    let form = multipart::Form::new()
        .text("field1", "value1")
        .text("field2", "value2");
    let body = RequestBody::from_multipart(form);
    assert!(matches!(body, RequestBody::Multipart(_)));
    assert_eq!(body.content_length(), None);
}

#[test]
fn test_empty() {
    let body = RequestBody::empty();
    assert!(matches!(body, RequestBody::Empty));
    assert!(body.is_empty());
    assert_eq!(body.content_length(), Some(0));
}

#[test]
fn test_from_reqwest_body() {
    let reqwest_body = reqwest::Body::from("test");
    let body = RequestBody::from_reqwest_body(reqwest_body);
    assert!(matches!(body, RequestBody::Custom(_)));
    assert_eq!(body.content_length(), None);
}

#[test]
fn test_display_formatting() {
    assert_eq!(RequestBody::empty().to_string(), "empty body");
    assert_eq!(
        RequestBody::from_static("test").to_string(),
        "static (4 bytes)"
    );
    assert_eq!(
        RequestBody::from_string("test").to_string(),
        "string (4 bytes)"
    );
    assert_eq!(
        RequestBody::from_bytes(Bytes::from("test")).to_string(),
        "bytes (4 bytes)"
    );
    assert_eq!(
        RequestBody::from_vec(vec![1, 2, 3]).to_string(),
        "binary (3 bytes)"
    );
    assert_eq!(RequestBody::from_json(json!({})).to_string(), "JSON");
    assert_eq!(
        RequestBody::from_form(vec![("a".to_string(), "b".to_string())]).to_string(),
        "form (1 fields)"
    );

    let form = multipart::Form::new().text("field", "value");
    assert_eq!(RequestBody::from_multipart(form).to_string(), "multipart");

    let reqwest_body = reqwest::Body::from("test");
    assert_eq!(
        RequestBody::from_reqwest_body(reqwest_body).to_string(),
        "custom"
    );
}

#[test]
fn test_debug_sensitive_keys() {
    let sensitive_keys = [
        "password",
        "PASSWORD",
        "Password",
        "passWord",
        "token",
        "TOKEN",
        "Token",
        "access_token",
        "secret",
        "SECRET",
        "Secret",
        "client_secret",
        "auth",
        "AUTH",
        "Auth",
        "authorization",
    ];
    for key in &sensitive_keys {
        let form_body =
            RequestBody::from_form(vec![(key.to_string(), "sensitive_value".to_string())]);
        let debug_str = format!("{form_body:?}",);
        assert!(
            debug_str.contains("[REDACTED]"),
            "Key '{key}' should be redacted",
        );
    }
}

#[test]
fn test_debug_long_strings() {
    let long_string = "a".repeat(150);
    let body = RequestBody::from_static(Box::leak(long_string.clone().into_boxed_str()));
    let debug_str = format!("{body:?}",);
    assert!(debug_str.contains("..."));
    assert!(debug_str.contains("+50 more chars"));
}

#[test]
fn test_content_length_edge_cases() {
    let complex_json = json!({
        "array": [1, 2, 3],
        "object": {"nested": "value"},
        "string": "test"
    });
    let body = RequestBody::from_json(complex_json);
    assert!(body.content_length().is_some());

    let form_data = vec![
        ("field1".to_string(), "value1".to_string()),
        ("field2".to_string(), "value with spaces".to_string()),
    ];
    let body = RequestBody::from_form(form_data);
    assert!(body.content_length().is_some());
}
