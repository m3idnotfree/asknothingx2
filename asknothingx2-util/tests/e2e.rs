#![cfg(all(test, feature = "api", feature = "stream"))]
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use asknothingx2_util::api::{
    preset,
    request::{RequestBody, RequestParts},
};
use bytes::Bytes;
use http::Method;
use reqwest::redirect::Policy;
use url::Url;
use wiremock::{
    matchers::{method, path},
    Match, Mock, MockServer, Request, ResponseTemplate,
};

#[derive(Debug, Clone)]
struct BodyCapture {
    captured_body: Arc<Mutex<Option<Vec<u8>>>>,
}

impl BodyCapture {
    fn new() -> Self {
        Self {
            captured_body: Arc::new(Mutex::new(None)),
        }
    }

    fn get_captured_body(&self) -> Option<Vec<u8>> {
        self.captured_body.lock().unwrap().clone()
    }
}

impl Match for BodyCapture {
    fn matches(&self, request: &Request) -> bool {
        let body = request.body.clone();
        *self.captured_body.lock().unwrap() = Some(body);
        true
    }
}

#[tokio::test]
async fn get() {
    let mock_server = MockServer::start().await;

    Mock::given(method(Method::GET))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_string("success"))
        .mount(&mock_server)
        .await;

    let config = preset::for_test("test/1.0");
    let client = config.build_client().unwrap();

    let url = Url::parse(&format!("{}/test", mock_server.uri())).unwrap();
    let request_parts = RequestParts::new(Method::GET, url);
    let request_bueilder = request_parts.into_request_builder(&client).unwrap();

    let response = request_bueilder.send().await.unwrap();

    assert_eq!(200, response.status());

    let body = response.text().await.unwrap();
    assert_eq!("success", body);
}

#[tokio::test]
async fn delay() {
    let mock_server = MockServer::start().await;

    Mock::given(method(Method::GET))
        .and(path("/slow"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(3)))
        .mount(&mock_server)
        .await;

    let config =
        preset::for_test("test/1.0").timeouts(Duration::from_secs(2), Duration::from_millis(300));
    let client = config.build_client().unwrap();

    let url = Url::parse(&format!("{}/slow", mock_server.uri())).unwrap();
    let request_parts = RequestParts::new(Method::GET, url);
    let request_bueilder = request_parts.into_request_builder(&client).unwrap();

    let response = request_bueilder.send().await;

    assert!(response.is_err());
    assert!(response.unwrap_err().is_timeout());
}

#[tokio::test]
async fn redirect() {
    let mock_server = MockServer::start().await;

    Mock::given(method(Method::GET))
        .and(path("/redirect1"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/redirect2"))
        .mount(&mock_server)
        .await;
    Mock::given(method(Method::GET))
        .and(path("/redirect2"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/redirect3"))
        .mount(&mock_server)
        .await;
    Mock::given(method(Method::GET))
        .and(path("/redirect3"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/redirect4"))
        .mount(&mock_server)
        .await;
    Mock::given(method(Method::GET))
        .and(path("/redirect4"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/redirect5"))
        .mount(&mock_server)
        .await;
    Mock::given(method(Method::GET))
        .and(path("/redirect5"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/final"))
        .mount(&mock_server)
        .await;
    Mock::given(method(Method::GET))
        .and(path("/final"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let config = preset::for_test("test/1.0").redirect(Policy::limited(3));
    let client = config.build_client().unwrap();

    let url = Url::parse(&format!("{}/redirect1", mock_server.uri())).unwrap();
    let request_parts = RequestParts::new(Method::GET, url);
    let request_bueilder = request_parts.into_request_builder(&client).unwrap();

    let response = request_bueilder.send().await;

    assert!(response.is_err());
    assert!(response.unwrap_err().is_redirect());
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn bytes_iter() {
    let mock_server = MockServer::start().await;
    let body_capture = BodyCapture::new();

    Mock::given(method(Method::POST))
        .and(path("/upload"))
        .and(body_capture.clone())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let chunks = vec![
        Bytes::from("chunk1"),
        Bytes::from("chunk2"),
        Bytes::from("chunk3"),
    ];
    let body = RequestBody::from_bytes_iter(chunks.clone());
    let config = preset::for_test("test/1.0")
        .timeouts(Duration::from_millis(500), Duration::from_millis(100));
    let client = config.build_client().unwrap();

    let url = Url::parse(&format!("{}/upload", mock_server.uri())).unwrap();
    let mut request_parts = RequestParts::new(Method::POST, url);
    request_parts.body(body);

    let request_bueilder = request_parts.into_request_builder(&client).unwrap();

    let response = request_bueilder.send().await.unwrap();
    assert_eq!(response.status(), 200);

    let captured = body_capture.get_captured_body().unwrap();
    let expected = chunks.into_iter().fold(Vec::new(), |mut acc, chunk| {
        acc.extend_from_slice(&chunk);
        acc
    });

    assert_eq!(captured, expected);
    assert_eq!(String::from_utf8(captured).unwrap(), "chunk1chunk2chunk3");
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn file_stream() {
    use std::io::Write;

    use tempfile::NamedTempFile;

    let mock_server = MockServer::start().await;
    let body_capture = BodyCapture::new();

    Mock::given(method(Method::POST))
        .and(path("/file"))
        .and(body_capture.clone())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let file_content = "This is test file content for streaming upload.";
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(file_content.as_bytes()).unwrap();

    let file = tokio::fs::File::open(temp_file.path()).await.unwrap();

    let body = RequestBody::from_file(file);
    let config = preset::for_test("test/1.0")
        .timeouts(Duration::from_millis(500), Duration::from_millis(100));
    let client = config.build_client().unwrap();

    let url = Url::parse(&format!("{}/file", mock_server.uri())).unwrap();
    let mut request_parts = RequestParts::new(Method::POST, url);
    request_parts.body(body);

    let request_bueilder = request_parts.into_request_builder(&client).unwrap();

    let response = request_bueilder.send().await.unwrap();
    assert_eq!(response.status(), 200);

    let captured = body_capture.get_captured_body().unwrap();
    let captured_str = String::from_utf8(captured).unwrap();

    assert_eq!(captured_str, file_content);
}
