use asknothingx2_util::api::{request::HeaderError, AuthScheme, HeaderBuilder};
use http::{
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CACHE_CONTROL,
        CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT,
    },
    HeaderName, HeaderValue,
};

// Test construction methods
#[test]
fn test_new() {
    let builder = HeaderBuilder::new();
    let headers = builder.build();
    assert!(headers.is_empty());
}

#[test]
fn test_with_capacity() {
    let builder = HeaderBuilder::with_capacity(10);
    let headers = builder.build();
    assert!(headers.is_empty());
    assert!(headers.capacity() >= 10);
}

// Test generic header methods
#[test]
fn test_header() {
    let mut builder = HeaderBuilder::new();
    let custom_header = HeaderName::from_static("x-custom");
    let custom_value = HeaderValue::from_static("test-value");

    builder.header(custom_header.clone(), custom_value.clone());
    let headers = builder.build();

    assert_eq!(headers.get(&custom_header), Some(&custom_value));
}

#[test]
fn test_header_static() {
    let mut builder = HeaderBuilder::new();
    let custom_header = HeaderName::from_static("x-custom");

    builder.header_static(custom_header.clone(), "test-value");
    let headers = builder.build();

    assert_eq!(headers.get(&custom_header).unwrap(), "test-value");
}

#[test]
fn test_header_str_success() {
    let mut builder = HeaderBuilder::new();
    let custom_header = HeaderName::from_static("x-custom");

    let result = builder.header_str(custom_header.clone(), "test-value");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get(&custom_header).unwrap(), "test-value");
}

#[test]
fn test_header_str_invalid() {
    let mut builder = HeaderBuilder::new();
    let custom_header = HeaderName::from_static("x-custom");

    // Test with invalid header value (contains null byte)
    let result = builder.header_str(custom_header, "test\0value");
    assert!(result.is_err());

    if let Err(HeaderError::InvalidHeaderValue {
        name,
        value,
        reason: _,
    }) = result
    {
        assert_eq!(name, "x-custom");
        assert_eq!(value, "test\0value");
    } else {
        panic!("Expected InvalidHeaderValue error");
    }
}

#[test]
fn test_append() {
    let mut builder = HeaderBuilder::new();
    let custom_header = HeaderName::from_static("x-custom");
    let value1 = HeaderValue::from_static("value1");
    let value2 = HeaderValue::from_static("value2");

    builder.header(custom_header.clone(), value1);
    let result = builder.append(custom_header.clone(), value2);
    assert!(result.is_ok());

    let headers = builder.build();
    let values: Vec<_> = headers.get_all(&custom_header).iter().collect();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], "value1");
    assert_eq!(values[1], "value2");
}

// Test specific header methods
#[test]
fn test_client_id() {
    let mut builder = HeaderBuilder::new();
    builder.client_id("test-client-123").unwrap();

    let headers = builder.build();
    assert_eq!(headers.get("client-id").unwrap(), "test-client-123");
}

#[test]
fn test_user_agent() {
    let mut builder = HeaderBuilder::new();
    builder.user_agent("MyApp/1.0").unwrap();

    let headers = builder.build();
    assert_eq!(headers.get(USER_AGENT).unwrap(), "MyApp/1.0");
}

#[test]
fn test_cache_control_no_cache() {
    let mut builder = HeaderBuilder::new();
    builder.cache_control_no_cache();

    let headers = builder.build();
    assert_eq!(headers.get(CACHE_CONTROL).unwrap(), "no-cache");
}

#[test]
fn test_cache_control_custom() {
    let mut builder = HeaderBuilder::new();
    let result = builder.cache_control("max-age=3600");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get(CACHE_CONTROL).unwrap(), "max-age=3600");
}

#[test]
fn test_cache_control_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.cache_control("invalid\0value");
    assert!(result.is_err());
}

#[test]
fn test_api_key() {
    let mut builder = HeaderBuilder::new();
    let result = builder.api_key("secret-key-123");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get("x-api-key").unwrap(), "secret-key-123");
}

#[test]
fn test_api_key_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.api_key("invalid\0key");
    assert!(result.is_err());
}

#[test]
fn test_request_id() {
    let mut builder = HeaderBuilder::new();
    let result = builder.request_id("req-123-456");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get("x-request-id").unwrap(), "req-123-456");
}

#[test]
fn test_request_id_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.request_id("invalid\0id");
    assert!(result.is_err());
}

#[test]
fn test_origin() {
    let mut builder = HeaderBuilder::new();
    let result = builder.origin("https://example.com");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get(ORIGIN).unwrap(), "https://example.com");
}

#[test]
fn test_origin_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.origin("invalid\0origin");
    assert!(result.is_err());
}

#[test]
fn test_referer() {
    let mut builder = HeaderBuilder::new();
    let result = builder.referer("https://referrer.com");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get(REFERER).unwrap(), "https://referrer.com");
}

#[test]
fn test_referer_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.referer("invalid\0referer");
    assert!(result.is_err());
}

// Test CORS methods
#[test]
fn test_cors_allow_all() {
    let mut builder = HeaderBuilder::new();
    builder.cors_allow_all();

    let headers = builder.build();
    assert_eq!(headers.get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "*");
}

#[test]
fn test_cors_allow_origin() {
    let mut builder = HeaderBuilder::new();
    let result = builder.cors_allow_origin("https://trusted.com");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(
        headers.get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "https://trusted.com"
    );
}

#[test]
fn test_cors_allow_origin_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.cors_allow_origin("invalid\0origin");
    assert!(result.is_err());
}

#[test]
fn test_cors_allow_methods_standard() {
    let mut builder = HeaderBuilder::new();
    builder.cors_allow_methods_standard();

    let headers = builder.build();
    assert_eq!(
        headers.get(ACCESS_CONTROL_ALLOW_METHODS).unwrap(),
        "GET,POST,PUT,DELETE"
    );
}

#[test]
fn test_cors_allow_headers_standard() {
    let mut builder = HeaderBuilder::new();
    builder.cors_allow_headers_standard();

    let headers = builder.build();
    assert_eq!(
        headers.get(ACCESS_CONTROL_ALLOW_HEADERS).unwrap(),
        "Content-Type,Authorization"
    );
}

// Test connection methods
#[test]
fn test_connection_keep_alive() {
    let mut builder = HeaderBuilder::new();
    builder.connection_keep_alive();

    let headers = builder.build();
    assert_eq!(headers.get(CONNECTION).unwrap(), "keep-alive");
}

#[test]
fn test_connection_close() {
    let mut builder = HeaderBuilder::new();
    builder.connection_close();

    let headers = builder.build();
    assert_eq!(headers.get(CONNECTION).unwrap(), "close");
}

#[test]
fn test_content_length() {
    let mut builder = HeaderBuilder::new();
    builder.content_length(1024);

    let headers = builder.build();
    assert_eq!(headers.get(CONTENT_LENGTH).unwrap(), "1024");
}

// Test combination methods
#[test]
fn test_json_api() {
    let mut builder = HeaderBuilder::new();
    builder.json_api();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT).unwrap(), "application/json");
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
}

// Test accept methods
#[test]
fn test_accept_json() {
    let mut builder = HeaderBuilder::new();
    builder.accept_json();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT).unwrap(), "application/json");
}

#[test]
fn test_accept_html() {
    let mut builder = HeaderBuilder::new();
    builder.accept_html();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT).unwrap(), "text/html");
}

#[test]
fn test_accept_text() {
    let mut builder = HeaderBuilder::new();
    builder.accept_text();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT).unwrap(), "text/plain");
}

#[test]
fn test_accept_any() {
    let mut builder = HeaderBuilder::new();
    builder.accept_any();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT).unwrap(), "*/*");
}

#[test]
fn test_accept_encoding_standard() {
    let mut builder = HeaderBuilder::new();
    builder.accept_encoding_standard();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT_ENCODING).unwrap(), "gzip,deflate,br");
}

#[test]
fn test_accept_language_en() {
    let mut builder = HeaderBuilder::new();
    builder.accept_language_en();

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT_LANGUAGE).unwrap(), "en-US,en;q=0.9");
}

#[test]
fn test_accept_language_custom() {
    let mut builder = HeaderBuilder::new();
    let result = builder.accept_language("fr-FR,fr;q=0.9");
    assert!(result.is_ok());

    let headers = builder.build();
    assert_eq!(headers.get(ACCEPT_LANGUAGE).unwrap(), "fr-FR,fr;q=0.9");
}

#[test]
fn test_accept_language_invalid() {
    let mut builder = HeaderBuilder::new();
    let result = builder.accept_language("invalid\0lang");
    assert!(result.is_err());
}

// Test content-type methods
#[test]
fn test_content_type_formencoded() {
    let mut builder = HeaderBuilder::new();
    builder.content_type_formencoded();

    let headers = builder.build();
    assert_eq!(
        headers.get(CONTENT_TYPE).unwrap(),
        "application/x-www-form-urlencoded"
    );
}

#[test]
fn test_content_type_json() {
    let mut builder = HeaderBuilder::new();
    builder.content_type_json();

    let headers = builder.build();
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
}

#[test]
fn test_content_type_text() {
    let mut builder = HeaderBuilder::new();
    builder.content_type_text();

    let headers = builder.build();
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/plain");
}

#[test]
fn test_content_type_html() {
    let mut builder = HeaderBuilder::new();
    builder.content_type_html();

    let headers = builder.build();
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/html");
}

#[test]
fn test_content_type_multipart() {
    let mut builder = HeaderBuilder::new();
    builder.content_type_multipart();

    let headers = builder.build();
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "multipart/form-data");
}

// Test authorization methods
#[test]
fn test_basic_auth() {
    let mut builder = HeaderBuilder::new();
    builder.basic_auth("user", "pass");

    let headers = builder.build();
    let auth_header = headers.get(AUTHORIZATION).unwrap();

    // The basic auth should be "Basic " + base64("user:pass")
    // user:pass in base64 is dXNlcjpwYXNz
    assert_eq!(auth_header, "Basic dXNlcjpwYXNz");
}

#[test]
fn test_bearer_token() {
    let mut builder = HeaderBuilder::new();
    builder.bearer_token("abc123token");

    let headers = builder.build();
    assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Bearer abc123token");
}

#[test]
fn test_authorization_custom() {
    let mut builder = HeaderBuilder::new();
    let auth_scheme = AuthScheme::custom("Custom", "credential");
    builder.authorization(auth_scheme);

    let headers = builder.build();
    assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Custom credential");
}

// Test method chaining
#[test]
fn test_method_chaining() {
    let mut header = HeaderBuilder::new();

    header
        .user_agent("TestAgent/1.0")
        .unwrap()
        .accept_json()
        .content_type_json()
        .cache_control_no_cache()
        .connection_keep_alive()
        .cors_allow_all()
        .basic_auth("user", "pass");
    // .build();
    //
    let headers = header.build();

    assert_eq!(headers.get(USER_AGENT).unwrap(), "TestAgent/1.0");
    assert_eq!(headers.get(ACCEPT).unwrap(), "application/json");
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
    assert_eq!(headers.get(CACHE_CONTROL).unwrap(), "no-cache");
    assert_eq!(headers.get(CONNECTION).unwrap(), "keep-alive");
    assert_eq!(headers.get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "*");
    assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Basic dXNlcjpwYXNz");
}

// Test error propagation in chaining
#[test]
fn test_error_in_chain() {
    let mut builder = HeaderBuilder::new();
    let result = builder
        .user_agent("TestAgent/1.0")
        .unwrap()
        .header_str(HeaderName::from_static("x-custom"), "invalid\0value");

    assert!(result.is_err());
}

// Test header replacement
#[test]
fn test_header_replacement() {
    let mut builder = HeaderBuilder::new();
    builder.user_agent("First").unwrap();
    builder.user_agent("Second").unwrap();

    let headers = builder.build();
    assert_eq!(headers.get(USER_AGENT).unwrap(), "Second");
}

// Test multiple different headers
#[test]
fn test_multiple_headers() {
    let mut headers = HeaderBuilder::new();

    headers
        .client_id("client-123")
        .unwrap()
        .api_key("key-456")
        .unwrap()
        .request_id("req-789")
        .unwrap()
        .origin("https://example.com")
        .unwrap()
        .referer("https://referrer.com")
        .unwrap()
        .user_agent("TestApp/2.0")
        .unwrap()
        .content_length(500);

    let headers = headers.build();
    assert_eq!(headers.get("client-id").unwrap(), "client-123");
    assert_eq!(headers.get("x-api-key").unwrap(), "key-456");
    assert_eq!(headers.get("x-request-id").unwrap(), "req-789");
    assert_eq!(headers.get(ORIGIN).unwrap(), "https://example.com");
    assert_eq!(headers.get(REFERER).unwrap(), "https://referrer.com");
    assert_eq!(headers.get(USER_AGENT).unwrap(), "TestApp/2.0");
    assert_eq!(headers.get(CONTENT_LENGTH).unwrap(), "500");
    assert_eq!(headers.len(), 7);
}

// Test edge cases
#[test]
fn test_empty_values() {
    let mut headers = HeaderBuilder::new();
    headers
        .client_id("")
        .unwrap()
        .user_agent("")
        .unwrap()
        .bearer_token("");

    let headers = headers.build();

    assert_eq!(headers.get("client-id").unwrap(), "");
    assert_eq!(headers.get(USER_AGENT).unwrap(), "");
    assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Bearer ");
}

#[test]
fn test_unicode_values() {
    let mut builder = HeaderBuilder::new();
    // Note: HTTP headers should be ASCII, but let's test the builder's behavior
    let result = builder.header_str(HeaderName::from_static("x-test"), "café");
    // This should succeed as "café" contains valid UTF-8 that's also valid for HTTP headers
    assert!(result.is_ok());
}

#[test]
fn test_large_header_count() {
    let mut builder = HeaderBuilder::with_capacity(100);

    for i in 0..50 {
        let header_name = HeaderName::try_from(format!("x-custom-{i}")).unwrap();
        let header_value = HeaderValue::from_str(&format!("value-{i}")).unwrap();
        builder.header(header_name, header_value);
    }

    let headers = builder.build();
    assert_eq!(headers.len(), 50);

    for i in 0..50 {
        let header_name = format!("x-custom-{i}");
        let expected_value = format!("value-{i}");
        let header_value = headers
            .get(&header_name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // assert_eq!(headers.get(&header_name).unwrap(), expected_value);
        assert_eq!(header_value, expected_value);
    }
}

// Additional tests for AuthScheme integration
#[cfg(test)]
mod auth_scheme_tests {
    use asknothingx2_util::api::{AuthScheme, DigestBuilder, HeaderBuilder, SCRAMVariant};
    use http::header::AUTHORIZATION;

    #[test]
    fn test_digest_auth() {
        let digest = DigestBuilder::new("user", "realm", "nonce123", "/api/test", "response456")
            .algorithm("MD5")
            .qop("auth")
            .cnonce("cnonce789")
            .nc("00000001");

        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::digest(digest));

        let headers = builder.build();
        let auth_value = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();

        assert!(auth_value.starts_with("Digest "));
        assert!(auth_value.contains("username=\"user\""));
        assert!(auth_value.contains("realm=\"realm\""));
        assert!(auth_value.contains("nonce=\"nonce123\""));
        assert!(auth_value.contains("uri=\"/api/test\""));
        assert!(auth_value.contains("response=\"response456\""));
        assert!(auth_value.contains("algorithm=MD5"));
        assert!(auth_value.contains("qop=auth"));
        assert!(auth_value.contains("cnonce=\"cnonce789\""));
        assert!(auth_value.contains("nc=00000001"));
    }

    #[test]
    fn test_hoba_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::hoba("result123"));

        let headers = builder.build();
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap(),
            "HOBA result=\"result123\""
        );
    }

    #[test]
    fn test_mutual_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::mutual("credentials123"));

        let headers = builder.build();
        assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Mutual credentials123");
    }

    #[test]
    fn test_negotiate_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::negotiate("token123"));

        let headers = builder.build();
        assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Negotiate token123");
    }

    #[test]
    fn test_vapid_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::vapid(
            "pubkey",
            "mailto:test@example.com",
            "signature",
        ));

        let headers = builder.build();
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap(),
            "VAPID k=pubkey, a=mailto:test@example.com, s=signature"
        );
    }

    #[test]
    fn test_scram_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::scram(SCRAMVariant::SHA256, "credentials"));

        let headers = builder.build();
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap(),
            "SCRAM-SHA-256 credentials"
        );
    }

    #[test]
    fn test_aws4_auth() {
        let mut builder = HeaderBuilder::new();
        builder.authorization(AuthScheme::aws4_hmac_sha256(
            "AKIAIOSFODNN7EXAMPLE",
            "signature123",
            "us-east-1",
            "s3",
            "20230101T000000Z",
        ));

        let headers = builder.build();
        let auth_value = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();

        assert!(auth_value.starts_with("AWS4-HMAC-SHA256 "));
        assert!(auth_value.contains(
            "Credential=AKIAIOSFODNN7EXAMPLE/20230101T000000Z/us-east-1/s3/aws4_request"
        ));
        assert!(auth_value.contains("SignedHeaders=host;x-amz-date"));
        assert!(auth_value.contains("Signature=signature123"));
    }
}
