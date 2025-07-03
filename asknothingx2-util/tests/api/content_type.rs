use asknothingx2_util::api::content_type::*;

// =============================================================================
// ALIAS TESTING
// =============================================================================

#[test]
fn test_all_aliases() {
    // Test specific known aliases
    assert_eq!("text/x-markdown".parse::<Text>(), Ok(Text::Markdown));
    assert_eq!("image/jpg".parse::<Image>(), Ok(Image::Jpeg));
    assert_eq!("video/avi".parse::<Video>(), Ok(Video::XMsvideo));
    assert_eq!("application/font-woff".parse::<Font>(), Ok(Font::Woff));

    // Ensure primary and alias parse to same result
    assert_eq!(
        "text/markdown".parse::<Text>(),
        "text/x-markdown".parse::<Text>()
    );
    assert_eq!("image/jpeg".parse::<Image>(), "image/jpg".parse::<Image>());
}

// =============================================================================
// EXTENSION MAPPING TESTS
// =============================================================================

#[test]
fn test_extension_mappings() {
    // Test known extension mappings
    assert_eq!(Application::from_extension("json"), Some(Application::Json));
    assert_eq!(Application::from_extension("pdf"), Some(Application::Pdf));
    assert_eq!(Application::from_extension("zip"), Some(Application::Zip));

    assert_eq!(Text::from_extension("txt"), Some(Text::Plain));
    assert_eq!(Text::from_extension("html"), Some(Text::Html));
    assert_eq!(Text::from_extension("css"), Some(Text::Css));

    assert_eq!(Image::from_extension("jpg"), Some(Image::Jpeg));
    assert_eq!(Image::from_extension("png"), Some(Image::Png));
    assert_eq!(Image::from_extension("gif"), Some(Image::Gif));

    // Test case insensitive
    assert_eq!(Application::from_extension("JSON"), Some(Application::Json));
    assert_eq!(Text::from_extension("HTML"), Some(Text::Html));
    assert_eq!(Image::from_extension("PNG"), Some(Image::Png));

    // Test unknown extensions
    assert_eq!(Application::from_extension("unknown"), None);
    assert_eq!(Text::from_extension("xyz"), None);
    assert_eq!(Image::from_extension("abc"), None);
}

// =============================================================================
// CONTENT TYPE UTILITY TESTS
// =============================================================================

#[test]
fn test_content_type_utilities() {
    // Test from_path
    assert_eq!(
        ContentType::from_path("file.json"),
        Some(ContentType::Application(Application::Json))
    );
    assert_eq!(
        ContentType::from_path("/path/to/document.pdf"),
        Some(ContentType::Application(Application::Pdf))
    );
    assert_eq!(
        ContentType::from_path("image.jpg"),
        Some(ContentType::Image(Image::Jpeg))
    );
    assert_eq!(ContentType::from_path("noext"), None);

    // Test from_filename
    assert_eq!(
        ContentType::from_filename("data.csv"),
        Some(ContentType::Text(Text::Csv))
    );
    assert_eq!(
        ContentType::from_filename("style.css"),
        Some(ContentType::Text(Text::Css))
    );
    assert_eq!(
        ContentType::from_filename("video.mp4"),
        Some(ContentType::Video(Video::Mp4))
    );
    assert_eq!(ContentType::from_filename("noextension"), None);

    // Test is_extension_supported
    assert!(ContentType::is_extension_supported("json"));
    assert!(ContentType::is_extension_supported("html"));
    assert!(ContentType::is_extension_supported("jpg"));
    assert!(!ContentType::is_extension_supported("unknown"));
}

// =============================================================================
// PARAMETER EXTRACTION TESTS
// =============================================================================

#[test]
fn test_parameter_extraction() {
    // Test charset extraction
    assert_eq!(
        ContentType::extract_charset("text/html; charset=utf-8"),
        Some("utf-8")
    );
    assert_eq!(
        ContentType::extract_charset("text/html; charset=\"utf-8\""),
        Some("utf-8")
    );
    assert_eq!(ContentType::extract_charset("text/html"), None);

    // Test boundary extraction
    assert_eq!(
        ContentType::extract_boundary("multipart/form-data; boundary=test123"),
        Some("test123")
    );
    assert_eq!(
        ContentType::extract_boundary("multipart/form-data; boundary=\"test123\""),
        Some("test123")
    );
    assert_eq!(ContentType::extract_boundary("text/html"), None);

    // Test matches_with_params
    assert!(ContentType::matches_with_params(
        "text/html; charset=utf-8",
        ContentType::Text(Text::Html)
    ));
    assert!(ContentType::matches_with_params(
        "application/json",
        ContentType::Application(Application::Json)
    ));
    assert!(!ContentType::matches_with_params(
        "text/plain",
        ContentType::Text(Text::Html)
    ));
}

// =============================================================================
// HEADER MANIPULATION TESTS
// =============================================================================

#[test]
fn test_header_manipulation() {
    use http::HeaderMap;

    let content_type = ContentType::Text(Text::Html);
    let mut headers = HeaderMap::new();

    content_type.set_on_headers(&mut headers);

    assert!(headers.contains_key("content-type"));
    let header_value = headers.get("content-type").unwrap();
    assert_eq!(header_value.to_str().unwrap(), "text/html");
}

// =============================================================================
// CONST FUNCTION TESTS
// =============================================================================

#[test]
fn test_const_functions() {
    // These should work in const context
    const APP_STR: &str = Application::Json.as_str();
    const APP_STATIC: &str = Application::Json.as_static();
    const APP_EXTENSIONS: &[&str] = Application::Json.extensions();
    const APP_PRIMARY: Option<&str> = Application::Json.primary_extension();

    assert_eq!(APP_STR, "application/json");
    assert_eq!(APP_STATIC, "application/json");
    assert_eq!(APP_EXTENSIONS, &["json"]);
    assert_eq!(APP_PRIMARY, Some("json"));
}

// #[test]
// fn invalid_content_types() {
//     let inputs = vec![
//         // ===== EMPTY/WHITESPACE =====
//         "".to_string(),
//         " ".to_string(),
//         "  ".to_string(),
//         "\t".to_string(),
//         "\n".to_string(),
//         "\r\n".to_string(),
//         // ===== MISSING PARTS =====
//         "invalid".to_string(),      // No slash
//         "application/".to_string(), // Missing subtype
//         "/json".to_string(),        // Missing main type
//         "/".to_string(),            // Just slash
//         "//".to_string(),           // Double slash only
//         // ===== MALFORMED STRUCTURE =====
//         "application//json".to_string(),      // Double slash
//         "application/json/extra".to_string(), // Too many parts
//         "application/json/".to_string(),      // Trailing slash
//         "/application/json".to_string(),      // Leading slash
//         "application\\json".to_string(),      // Wrong separator
//         // ===== INVALID CHARACTERS =====
//         "type\x00/subtype".to_string(), // Null byte in type
//         "type/sub\x00type".to_string(), // Null byte in subtype
//         "тип/подтип".to_string(),       // Non-ASCII characters
//         // "type/sub type".to_string(),    // Space in subtype
//         "ty pe/subtype".to_string(), // Space in type
//         "type@/subtype".to_string(), // Invalid character in type
//         "type/sub@type".to_string(), // Invalid character in subtype
//         // ===== MALFORMED PARAMETERS =====
//         "text/plain; =value".to_string(), // Missing parameter name
//         "text/plain; name=".to_string(),  // Missing parameter value
//         "text/plain; =".to_string(),      // Missing both
//         "text/plain;;".to_string(),       // Double semicolon
//         "text/plain; ;".to_string(),      // Empty parameter
//         "text/plain;name=value;".to_string(), // Trailing semicolon
//         "text/plain; name value".to_string(), // Missing equals
//         "text/plain; name==value".to_string(), // Double equals
//         "text/plain; na me=value".to_string(), // Space in parameter name
//         // ===== UNCLOSED QUOTES =====
//         "text/plain; charset=\"utf-8".to_string(), // Unclosed double quote
//         "text/plain; charset='utf-8".to_string(),  // Unclosed single quote
//         "text/plain; charset=\"utf-8'".to_string(), // Mismatched quotes
//         "text/plain; charset='utf-8\"".to_string(), // Mismatched quotes
//         // ===== VERY LONG VALUES =====
//         // format!("text/plain; charset={}", "a".repeat(1000)), // Very long parameter
//         // format!("{}/json", "a".repeat(100)),                 // Very long type
//         // format!("application/{}", "b".repeat(100)),          // Very long subtype
//         // ===== CONTROL CHARACTERS =====
//         "text/plain\r\n".to_string(),             // CRLF in content type
//         "text\r/plain".to_string(),               // CR in type
//         "text/pla\nin".to_string(),               // LF in subtype
//         "text/plain;\rcharset=utf-8".to_string(), // CR in parameters
//     ];
//
//     for input in inputs {
//         use std::str::FromStr;
//
//         assert!(ContentType::from_str(&input).is_err(), "{input}");
//     }
// }
