mod body;
mod comprehensive;
mod edge_cases;
mod property_tests;
mod stress_tests;

use asknothingx2_util::api::request::RequestParts;
use http::Method;
use serde_json::json;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
struct RequestPartsBodyTestCase {
    name: &'static str,
    body_setter: fn(&mut RequestParts) -> &mut RequestParts,
    expected_body_variant: &'static str,
    expected_content_type: Option<&'static str>,
}

#[test]
fn test_request_parts_body_methods_table_driven() {
    let url = Url::parse("https://example.com").unwrap();

    let test_cases = vec![
        RequestPartsBodyTestCase {
            name: "Text body",
            body_setter: |parts| parts.text("Hello, World!"),
            expected_body_variant: "String",
            expected_content_type: Some("text/plain"),
        },
        RequestPartsBodyTestCase {
            name: "JSON body",
            body_setter: |parts| parts.json(json!({"message": "hello"})),
            expected_body_variant: "Json",
            expected_content_type: Some("application/json"),
        },
        RequestPartsBodyTestCase {
            name: "Form body",
            body_setter: |parts| parts.form(vec![("key".to_string(), "value".to_string())]),
            expected_body_variant: "Form",
            expected_content_type: Some("application/x-www-form-urlencoded"),
        },
        RequestPartsBodyTestCase {
            name: "Form pairs body",
            body_setter: |parts| parts.form_pairs(vec![("name", "Alice"), ("age", "30")]),
            expected_body_variant: "Form",
            expected_content_type: Some("application/x-www-form-urlencoded"),
        },
        RequestPartsBodyTestCase {
            name: "Empty body",
            body_setter: |parts| parts.empty(),
            expected_body_variant: "Empty",
            expected_content_type: None,
        },
    ];

    for case in test_cases {
        let mut request_parts = RequestParts::new(Method::POST, url.clone());
        let request_parts = (case.body_setter)(&mut request_parts);

        if let Some(body) = &request_parts.body {
            let debug_str = format!("{body:?}",);
            assert!(
                debug_str.contains(case.expected_body_variant),
                "Body should contain '{}' for case: {}, but was: {}",
                case.expected_body_variant,
                case.name,
                debug_str
            );
        } else {
            panic!("Expected body for case: {}", case.name);
        }

        if let Some(expected_ct) = case.expected_content_type {
            let content_type = request_parts
                .headers
                .get("content-type")
                .and_then(|v| v.to_str().ok());
            assert_eq!(
                content_type,
                Some(expected_ct),
                "Failed content-type for case: {}",
                case.name
            );
        }
    }
}

#[derive(Debug)]
struct RequestPartsConfigTestCase {
    name: &'static str,
    configurator: fn(&mut RequestParts) -> &mut RequestParts,
    expected_timeout: Option<Duration>,
    expected_version: Option<http::Version>,
    expected_request_id: Option<&'static str>,
}

#[test]
fn test_request_parts_configuration_table_driven() {
    let url = Url::parse("https://example.com").unwrap();

    let test_cases = vec![
        RequestPartsConfigTestCase {
            name: "With timeout",
            configurator: |parts| parts.timeout(Duration::from_secs(30)),
            expected_timeout: Some(Duration::from_secs(30)),
            expected_version: None,
            expected_request_id: None,
        },
        RequestPartsConfigTestCase {
            name: "With HTTP version",
            configurator: |parts| parts.version(http::Version::HTTP_2),
            expected_timeout: None,
            expected_version: Some(http::Version::HTTP_2),
            expected_request_id: None,
        },
        RequestPartsConfigTestCase {
            name: "With request ID",
            configurator: |parts| parts.request_id("req-123"),
            expected_timeout: None,
            expected_version: None,
            expected_request_id: Some("req-123"),
        },
        RequestPartsConfigTestCase {
            name: "With all configurations",
            configurator: |parts| {
                parts
                    .timeout(Duration::from_secs(60))
                    .version(http::Version::HTTP_11)
                    .request_id("req-456")
            },
            expected_timeout: Some(Duration::from_secs(60)),
            expected_version: Some(http::Version::HTTP_11),
            expected_request_id: Some("req-456"),
        },
    ];

    for case in test_cases {
        let mut request_parts = RequestParts::new(Method::GET, url.clone());
        let request_parts = (case.configurator)(&mut request_parts);

        assert_eq!(
            request_parts.timeout, case.expected_timeout,
            "Failed timeout for case: {}",
            case.name
        );

        assert_eq!(
            request_parts.version, case.expected_version,
            "Failed version for case: {}",
            case.name
        );

        assert_eq!(
            request_parts.request_id.as_deref(),
            case.expected_request_id,
            "Failed request_id for case: {}",
            case.name
        );
    }
}

#[cfg(feature = "stream")]
mod stream_integration {
    use asknothingx2_util::api::request::RequestBody;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_file_streaming_integration() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = "Hello, this is test file content for streaming!";
        temp_file.write_all(test_content.as_bytes()).unwrap();

        let file_path = temp_file.path();
        let result = RequestBody::from_file_path(file_path).await;
        assert!(result.is_ok());

        let body = result.unwrap();
        assert!(!body.is_empty());

        let client = reqwest::Client::new();
        let builder = client.post("http://example.com");
        let _request = body.into_reqwest_body(builder).build().unwrap();
    }

    #[tokio::test]
    async fn test_buffered_file_streaming() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = "x".repeat(10000); // 10KB of data
        temp_file.write_all(test_content.as_bytes()).unwrap();

        let file_path = temp_file.path();
        let result = RequestBody::from_file_path_buffered(file_path, 1024).await;
        assert!(result.is_ok());

        let body = result.unwrap();
        assert!(!body.is_empty());
    }
}
