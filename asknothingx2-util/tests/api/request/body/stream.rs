#![cfg(all(test, feature = "stream", feature = "api"))]

use asknothingx2_util::api::request::{CodecType, RequestBody};
use bytes::Bytes;
use std::io::Cursor;
use tokio::process::Command;

#[tokio::test]
async fn test_from_file_path() {
    let result = RequestBody::from_file_path("tests/fixtures/test.txt").await;
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), RequestBody::File(_)));
}

#[tokio::test]
async fn test_from_file_path_not_found() {
    let result = RequestBody::from_file_path("/nonexistent/file.txt").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_from_file_path_buffered() {
    let result = RequestBody::from_file_path_buffered("tests/fixtures/test.txt", 1024).await;
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), RequestBody::BufferedFile { .. }));
}

#[test]
fn test_from_async_read() {
    let data = "async read data";
    let cursor = Cursor::new(data.as_bytes());
    let body = RequestBody::from_async_read(cursor);
    assert!(matches!(body, RequestBody::AsyncRead(_)));
}

#[tokio::test]
async fn test_from_command_output() {
    let mut command = Command::new("echo");
    command.arg("hello world");

    let result = RequestBody::from_command_output(command);

    assert!(result.is_ok());
    let body = result.unwrap();
    assert!(matches!(body, RequestBody::ProcessOutput { .. }));
}

#[test]
fn test_from_command_output_invalid() {
    let command = Command::new("nonexistent_command_12345");
    let result = RequestBody::from_command_output(command);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_iter() {
    let chunks = vec![
        Bytes::from("chunk1"),
        Bytes::from("chunk2"),
        Bytes::from("chunk3"),
    ];
    let body = RequestBody::from_bytes_iter(chunks.clone());
    assert!(matches!(body, RequestBody::BytesIterator(_)));
    assert_eq!(body.content_length(), Some(18));

    if let RequestBody::BytesIterator(stored_chunks) = body {
        assert_eq!(stored_chunks.len(), 3);
        assert_eq!(stored_chunks[0], Bytes::from("chunk1"));
    } else {
        panic!("Expected BytesIterator variant");
    }
}

#[test]
fn test_bytes_iterator_is_empty() {
    let empty_body = RequestBody::from_bytes_iter(Vec::<Bytes>::new());
    assert!(empty_body.is_empty());
}

#[test]
fn test_codec_types() {
    assert_eq!(CodecType::Bytes.to_string(), "bytes");
    assert_eq!(CodecType::Lines.to_string(), "lines");
    assert_eq!(CodecType::Json.to_string(), "json");
    assert_eq!(CodecType::Custom("xml".to_string()).to_string(), "xml");
}

#[test]
fn test_codec_equality() {
    assert_eq!(CodecType::Bytes, CodecType::Bytes);
    assert_eq!(CodecType::Lines, CodecType::Lines);
    assert_eq!(CodecType::Json, CodecType::Json);
    assert_eq!(
        CodecType::Custom("test".to_string()),
        CodecType::Custom("test".to_string())
    );
    assert_ne!(CodecType::Bytes, CodecType::Lines);
}

#[test]
fn test_from_framed_read() {
    let data = "line1\nline2\nline3";
    let cursor = Cursor::new(data.as_bytes());
    let body = RequestBody::from_framed_read(cursor, CodecType::Lines);
    assert!(matches!(body, RequestBody::CodecReader { .. }));
}

#[test]
fn test_from_bytes_framed() {
    let data = "binary data";
    let cursor = Cursor::new(data.as_bytes());
    let body = RequestBody::from_bytes_framed(cursor);
    assert!(matches!(body, RequestBody::CodecReader { .. }));
}
