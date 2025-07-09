use asknothingx2_util::api::request::{HeaderError, RequestBody, RequestParts};
use bytes::Bytes;
use http::Method;
use reqwest::{multipart, Client};
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use url::Url;

#[cfg(feature = "stream")]
use asknothingx2_util::api::request::CodecType;
#[cfg(feature = "stream")]
use std::io::Cursor;
#[cfg(feature = "stream")]
use tokio::process::Command;

#[tokio::test]
async fn test_into_reqwest_body_all_variants() {
    let client = Client::new();

    let body = RequestBody::from_static("hello world");
    let builder = client.post("http://example.com");
    let request = body.into_reqwest_body(builder).build().unwrap();
    assert_eq!(request.method(), &Method::POST);

    let body = RequestBody::from_string("test string".to_string());
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let body = RequestBody::from_bytes(Bytes::from("byte data"));
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let body = RequestBody::from_vec(vec![1, 2, 3, 4, 5]);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let body = RequestBody::from_json(json!({"key": "value", "number": 42}));
    let builder = client.post("http://example.com");
    let request = body.into_reqwest_body(builder).build().unwrap();

    assert!(request.body().is_some());

    let body = RequestBody::from_form(vec![
        ("name".to_string(), "John".to_string()),
        ("email".to_string(), "john@example.com".to_string()),
    ]);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let form = multipart::Form::new()
        .text("field1", "value1")
        .text("field2", "value2");
    let body = RequestBody::from_multipart(form);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let reqwest_body = reqwest::Body::from("custom body");
    let body = RequestBody::from_reqwest_body(reqwest_body);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let body = RequestBody::empty();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn test_into_reqwest_body_stream_variants() {
    let client = Client::new();

    let chunks = vec![
        Bytes::from("chunk1"),
        Bytes::from("chunk2"),
        Bytes::from("chunk3"),
    ];
    let body = RequestBody::from_bytes_iter(chunks);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let data = "async read data";
    let cursor = Cursor::new(data.as_bytes());
    let body = RequestBody::from_async_read(cursor);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let data = "line1\nline2\nline3";
    let cursor = Cursor::new(data.as_bytes());
    let body = RequestBody::from_framed_read(cursor, CodecType::Lines);
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();
}

#[tokio::test]
async fn test_real_http_request_json() {
    let client = Client::new();
    let json_data = json!({
        "name": "test",
        "value": 42,
        "nested": {
            "key": "value"
        }
    });

    let body = RequestBody::from_json(json_data.clone());
    let builder = client.post("https://httpbin.org/post");
    let request = body.into_reqwest_body(builder).build().unwrap();

    let response = timeout(Duration::from_secs(10), client.execute(request)).await;

    match response {
        Ok(Ok(resp)) => {
            assert!(resp.status().is_success());
            let response_text = resp.text().await.unwrap();
            assert!(response_text.contains("test"));
        }
        _ => {
            println!("httpbin.org unavailable, skipping network test");
        }
    }
}

#[tokio::test]
async fn test_real_http_request_form() {
    let client = Client::new();
    let form_data = vec![
        ("username".to_string(), "testuser".to_string()),
        ("password".to_string(), "testpass".to_string()),
    ];

    let body = RequestBody::from_form(form_data);
    let builder = client.post("https://httpbin.org/post");
    let request = body.into_reqwest_body(builder).build().unwrap();

    let response = timeout(Duration::from_secs(10), client.execute(request)).await;

    match response {
        Ok(Ok(resp)) => {
            assert!(resp.status().is_success());
        }
        _ => {
            println!("httpbin.org unavailable, skipping network test");
        }
    }
}

#[tokio::test]
async fn test_real_http_request_multipart() {
    let client = Client::new();
    let form = multipart::Form::new()
        .text("field1", "value1")
        .text("field2", "value2")
        .text("file_field", "file content here");

    let body = RequestBody::from_multipart(form);
    let builder = client.post("https://httpbin.org/post");
    let request = body.into_reqwest_body(builder).build().unwrap();

    let response = timeout(Duration::from_secs(10), client.execute(request)).await;

    match response {
        Ok(Ok(resp)) => {
            assert!(resp.status().is_success());
        }
        _ => {
            println!("httpbin.org unavailable, skipping network test");
        }
    }
}

#[test]
fn test_json_serialization_error() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), "value".to_string());

    let result = RequestBody::from_json_serializable(&map);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_json_content_length() {
    let complex_json = json!({
        "array": [1, 2, 3, 4, 5],
        "object": {"nested": {"deep": "value"}},
        "string": "test with special chars: 你好",
        "number": 42.7,
        "boolean": true,
        "null": null
    });

    let body = RequestBody::from_json(complex_json);
    let content_length = body.content_length();
    assert!(content_length.is_some());
    assert!(content_length.unwrap() > 0);
}

#[test]
fn test_form_encoding_edge_cases() {
    let form_data = vec![
        ("normal".to_string(), "value".to_string()),
        ("with spaces".to_string(), "value with spaces".to_string()),
        (
            "with&special=chars".to_string(),
            "value&with=special".to_string(),
        ),
        ("unicode".to_string(), "你好世界".to_string()),
        ("empty".to_string(), "".to_string()),
    ];

    let body = RequestBody::from_form(form_data);
    let content_length = body.content_length();
    assert!(content_length.is_some());
    assert!(content_length.unwrap() > 0);
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn test_file_not_found_error() {
    let result = RequestBody::from_file_path("/nonexistent/path/file.txt").await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(format!("{error}").contains("File"));
    assert!(format!("{error}").contains("open"));
}

#[cfg(feature = "stream")]
#[test]
fn test_invalid_command_error() {
    let mut command = Command::new("this_command_definitely_does_not_exist_12345");
    command.arg("test");

    let result = RequestBody::from_command_output(command);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(format!("{error}").contains("Process"));
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn test_large_bytes_iterator() {
    let mut large_chunks = Vec::new();
    for i in 0..1000 {
        large_chunks.push(Bytes::from(format!("chunk_{i:04}").repeat(100)));
    }

    let body = RequestBody::from_bytes_iter(large_chunks.clone());

    let expected_length: usize = large_chunks.iter().map(|b| b.len()).sum();
    assert_eq!(body.content_length(), Some(expected_length as u64));

    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn test_command_output_streaming() {
    let mut command = Command::new("echo");
    command.arg("hello world from command");

    let result = RequestBody::from_command_output(command);
    assert!(result.is_ok());

    let body = result.unwrap();

    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn test_codec_streaming() {
    let test_data = "line1\nline2\nline3\nline4";

    let cursor = Cursor::new(test_data.as_bytes());
    let body = RequestBody::from_framed_read(cursor, CodecType::Lines);
    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let cursor = Cursor::new(test_data.as_bytes());
    let body = RequestBody::from_bytes_framed(cursor);
    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let json_data = r#"{"line1": "value1"}
{"line2": "value2"}
{"line3": "value3"}"#;
    let cursor = Cursor::new(json_data.as_bytes());
    let body = RequestBody::from_framed_read(cursor, CodecType::Json);
    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();

    let cursor = Cursor::new(test_data.as_bytes());
    let body = RequestBody::from_framed_read(cursor, CodecType::Custom("xml".to_string()));
    let client = Client::new();
    let builder = client.post("http://example.com");
    let _request = body.into_reqwest_body(builder).build().unwrap();
}

#[test]
fn test_empty_data_variants() {
    assert!(RequestBody::from_static("").is_empty());
    assert!(RequestBody::from_string("").is_empty());
    assert!(RequestBody::from_bytes(Bytes::new()).is_empty());
    assert!(RequestBody::from_vec(vec![]).is_empty());
    assert!(RequestBody::from_form(vec![]).is_empty());
    assert!(RequestBody::from_json(json!(null)).is_empty());
    assert!(RequestBody::empty().is_empty());

    #[cfg(feature = "stream")]
    {
        let empty_chunks: Vec<Bytes> = vec![];
        assert!(RequestBody::from_bytes_iter(empty_chunks).is_empty());
    }
}

#[test]
fn test_very_large_static_strings() {
    let large_string = "x".repeat(1_000_000);
    let leaked_str: &'static str = Box::leak(large_string.into_boxed_str());
    let body = RequestBody::from_static(leaked_str);

    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(1_000_000));
}

#[test]
fn test_unicode_handling() {
    let unicode_data = "Hello 世界 🌍 Здравствуй мир";

    let body = RequestBody::from_string(unicode_data.to_string());
    assert!(!body.is_empty());
    assert!(body.content_length().unwrap() > unicode_data.chars().count() as u64);

    let json_body = RequestBody::from_json(json!({"message": unicode_data}));
    assert!(!json_body.is_empty());

    let form_body = RequestBody::from_form(vec![("message".to_string(), unicode_data.to_string())]);
    assert!(!form_body.is_empty());
}

#[test]
fn test_binary_data_handling() {
    let binary_data = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD, 0x80, 0x81];

    let body = RequestBody::from_vec(binary_data.clone());
    assert!(!body.is_empty());
    assert_eq!(body.content_length(), Some(binary_data.len() as u64));

    let bytes_body = RequestBody::from_bytes(Bytes::from(binary_data.clone()));
    assert!(!bytes_body.is_empty());
    assert_eq!(bytes_body.content_length(), Some(binary_data.len() as u64));
}

#[test]
fn test_request_parts_construction() {
    let url = Url::parse("https://example.com/api").unwrap();
    let parts = RequestParts::new(Method::POST, url.clone());

    assert_eq!(parts.method, Method::POST);
    assert_eq!(parts.url, url);
    assert!(parts.headers.is_empty());
    assert!(parts.body.is_none());
}

#[test]
fn test_request_parts_headers() {
    let url = Url::parse("https://example.com/api").unwrap();
    let mut parts = RequestParts::new(Method::GET, url);
    parts
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .header("User-Agent", "test-client/1.0");

    assert_eq!(parts.headers.len(), 3);
    assert_eq!(
        parts.headers.get("content-type").unwrap(),
        "application/json"
    );
    assert_eq!(
        parts.headers.get("authorization").unwrap(),
        "Bearer token123"
    );
}

#[test]
fn test_request_parts_try_header() {
    let url = Url::parse("https://example.com/api").unwrap();

    let mut result = RequestParts::new(Method::GET, url.clone());
    assert!(result
        .try_header("Content-Type", "application/json")
        .is_ok());

    let mut result = RequestParts::new(Method::GET, url.clone());
    assert!(result.try_header("Invalid Name", "value").is_err());

    let mut result = RequestParts::new(Method::GET, url);
    assert!(result.try_header("Valid-Name", "invalid\nvalue").is_err());
}

#[test]
fn test_request_parts_body_methods() {
    let url = Url::parse("https://example.com/api").unwrap();

    let mut parts = RequestParts::new(Method::POST, url.clone());
    parts.text("Hello, world!");
    assert!(parts.body.is_some());
    assert!(parts.headers.contains_key("content-type"));

    let mut parts = RequestParts::new(Method::POST, url.clone());
    parts.json(json!({"key": "value"}));
    assert!(parts.body.is_some());
    assert!(parts.headers.contains_key("content-type"));

    let mut parts = RequestParts::new(Method::POST, url.clone());
    parts.form(vec![("key".to_string(), "value".to_string())]);
    assert!(parts.body.is_some());
    assert!(parts.headers.contains_key("content-type"));

    let mut parts = RequestParts::new(Method::POST, url.clone());
    parts.form_pairs([("key1", "value1"), ("key2", "value2")]);
    assert!(parts.body.is_some());

    let mut parts = RequestParts::new(Method::POST, url);
    parts.empty();
    assert!(parts.body.is_some());
}

#[test]
fn test_request_parts_configuration() {
    let url = Url::parse("https://example.com/api").unwrap();
    let mut parts = RequestParts::new(Method::GET, url);
    parts
        .version(http::Version::HTTP_2)
        .timeout(Duration::from_secs(30))
        .request_id("req-123");

    assert_eq!(parts.version, Some(http::Version::HTTP_2));
    assert_eq!(parts.timeout, Some(Duration::from_secs(30)));
    assert_eq!(parts.request_id, Some("req-123".to_string()));
}

#[test]
fn test_request_parts_into_request_builder() {
    let url = Url::parse("https://example.com/api").unwrap();
    let client = Client::new();

    let mut parts = RequestParts::new(Method::POST, url);
    parts.header_mut().content_type_json();

    parts
        .json(json!({"test": "data"}))
        .timeout(Duration::from_secs(10))
        .request_id("test-request");

    let (builder, request_id) = parts.into_request_builder(&client);
    assert_eq!(request_id, Some("test-request".to_string()));

    let request = builder.build().unwrap();
    assert_eq!(request.method(), &Method::POST);
}

#[test]
fn test_decode_header_value() {
    use asknothingx2_util::api::request::decode_header_value;

    let result = decode_header_value("hello%20world");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello world");

    let result = decode_header_value("Hello%20%E4%B8%96%E7%95%8C");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello 世界");

    let result = decode_header_value("%FF%FE");
    assert!(result.is_err());
}

#[test]
fn test_encode_header() {
    use asknothingx2_util::api::request::encode_header;

    let result = encode_header("hello world");
    assert!(result.contains("%20"));

    let result = encode_header("test\"value");
    assert!(result.contains("%22"));

    let result = encode_header("世界");
    assert!(result.len() > 2);
}

#[test]
fn test_encode_rfc8187() {
    use asknothingx2_util::api::request::encode_rfc8187;

    let result = encode_rfc8187("hello_world");
    assert_eq!(result, "hello_world");

    let result = encode_rfc8187("世界");
    assert!(result.starts_with("utf-8''"));
    assert!(result.contains("%"));
}

#[test]
fn test_memory_usage_large_data() {
    let large_vec = vec![0u8; 10_000_000]; // 10MB
    let body = RequestBody::from_vec(large_vec.clone());

    assert_eq!(body.content_length(), Some(10_000_000));
}

#[tokio::test]
async fn test_concurrent_body_creation() {
    use tokio::task::JoinSet;

    let mut tasks = JoinSet::new();

    for i in 0..10 {
        tasks.spawn(async move {
            let data = format!("test data {i}").repeat(1000);
            let body = RequestBody::from_string(data);
            assert!(!body.is_empty());
            body
        });
    }

    let mut bodies = Vec::new();
    while let Some(result) = tasks.join_next().await {
        bodies.push(result.unwrap());
    }

    assert_eq!(bodies.len(), 10);
}

#[test]
fn test_debug_output_safety() {
    let sensitive_form = RequestBody::from_form(vec![
        ("username".to_string(), "john".to_string()),
        ("password".to_string(), "secret123".to_string()),
        ("api_token".to_string(), "sk-1234567890".to_string()),
        ("client_secret".to_string(), "very_secret".to_string()),
    ]);

    let debug_output = format!("{sensitive_form:?}");
    assert!(debug_output.contains("[REDACTED]"));
    assert!(!debug_output.contains("secret123"));
    assert!(!debug_output.contains("sk-1234567890"));
    assert!(!debug_output.contains("very_secret"));

    assert!(debug_output.contains("john"));
}

#[test]
fn test_comprehensive_display_output() {
    let test_cases = vec![
        (RequestBody::empty(), "empty body"),
        (RequestBody::from_static("test"), "static (4 bytes)"),
        (
            RequestBody::from_string("test".to_string()),
            "string (4 bytes)",
        ),
        (
            RequestBody::from_bytes(Bytes::from("test")),
            "bytes (4 bytes)",
        ),
        (RequestBody::from_vec(vec![1, 2, 3, 4]), "binary (4 bytes)"),
        (RequestBody::from_json(json!({"key": "value"})), "JSON"),
    ];

    for (body, expected) in test_cases {
        assert_eq!(body.to_string(), expected);
    }
}

#[cfg(feature = "stream")]
#[test]
fn test_stream_display_output() {
    let chunks = vec![Bytes::from("a"), Bytes::from("bb"), Bytes::from("ccc")];
    let body = RequestBody::from_bytes_iter(chunks);
    let display = body.to_string();
    assert!(display.contains("bytes iterator"));
    assert!(display.contains("3 chunks"));
    assert!(display.contains("6 bytes"));
}

#[test]
fn test_error_recovery_methods() {
    use asknothingx2_util::api::request::StreamError;

    let timeout_error = StreamError::timeout_error("test_operation", Duration::from_secs(5));
    assert!(timeout_error.is_recoverable());

    let custom_error = StreamError::custom("Custom error message");
    assert!(!custom_error.is_recoverable());

    assert!(timeout_error.is_temporary());
    assert!(!custom_error.is_temporary());
}

#[test]
fn test_header_error_types() {
    let errors = vec![
        HeaderError::InvalidHeaderName {
            name: "bad name".to_string(),
            reason: "contains space".to_string(),
        },
        HeaderError::InvalidHeaderValue {
            name: "good-name".to_string(),
            value: "bad\nvalue".to_string(),
            reason: "contains newline".to_string(),
        },
        HeaderError::EncodingFailed {
            name: "test-header".to_string(),
            reason: "encoding failed".to_string(),
        },
        HeaderError::InvalidUtf8 {
            reason: "invalid utf-8 sequence".to_string(),
        },
    ];

    for error in errors {
        let error_string = format!("{error}");
        assert!(!error_string.is_empty());
        let _debug = format!("{error:?}");
    }
}
