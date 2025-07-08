mod basic_auth_tests {
    use asknothingx2_util::api::AuthScheme;
    use http::HeaderValue;

    #[test]
    fn test_basic_auth_creation() {
        let auth = AuthScheme::basic("admin", "password123");

        assert_eq!(auth.scheme_name(), "Basic");
        assert_eq!(format!("{auth}"), "Basic (user: admin)");
    }

    #[test]
    fn test_basic_auth_header_value() {
        let auth = AuthScheme::basic("user", "pass");
        let header = auth.to_header_value().unwrap();

        // "user:pass" base64 encoded is "dXNlcjpwYXNz"
        assert_eq!(header, HeaderValue::from_static("Basic dXNlcjpwYXNz"));
    }

    #[test]
    fn test_basic_auth_with_special_characters() {
        let auth = AuthScheme::basic("user@domain.com", "p@ssw0rd!");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_auth_empty_fields() {
        let auth = AuthScheme::basic("", "password");
        let result = auth.to_header_value();
        assert!(result.is_ok());

        let auth = AuthScheme::basic("user", "");
        let result = auth.to_header_value();
        assert!(result.is_ok());

        let auth = AuthScheme::basic("", "");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_auth_with_unicode() {
        let auth = AuthScheme::basic("사용자", "비밀번호");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_auth_with_control_characters() {
        let auth = AuthScheme::basic("user\nname", "password");
        let result = auth.to_header_value();
        assert!(result.is_ok());

        let auth = AuthScheme::basic("username", "pass\rword");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_auth_long_credentials() {
        let long_username = "a".repeat(1000);
        let long_password = "b".repeat(1000);
        let auth = AuthScheme::basic(&long_username, &long_password);
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }
}

mod bearer_tests {
    use asknothingx2_util::api::AuthScheme;
    use http::HeaderValue;

    #[test]
    fn test_bearer_creation() {
        let auth = AuthScheme::bearer("abc123token");

        assert_eq!(auth.scheme_name(), "Bearer");
        assert_eq!(format!("{auth}"), "Bearer token");
    }

    #[test]
    fn test_bearer_header_value() {
        let auth = AuthScheme::bearer("my_secret_token");
        let header = auth.to_header_value().unwrap();

        assert_eq!(header, HeaderValue::from_static("Bearer my_secret_token"));
    }

    #[test]
    fn test_bearer_jwt_token() {
        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let auth = AuthScheme::bearer(jwt);
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_bearer_empty_token() {
        let auth = AuthScheme::bearer("");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_bearer_with_control_characters() {
        let auth = AuthScheme::bearer("token\nwith\nnewlines");
        let result = auth.to_header_value();
        assert!(result.is_err());
    }

    #[test]
    fn test_bearer_with_spaces() {
        let auth = AuthScheme::bearer("token with spaces");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }
}

mod digest_tests {
    use asknothingx2_util::api::{AuthScheme, DigestBuilder};

    #[test]
    fn test_digest_builder_basic() {
        let digest = DigestBuilder::new(
            "admin",
            "realm@example.com",
            "dcd98b7102dd2f0e8b11d0f600bfb0c093",
            "/dir/index.html",
            "6629fae49393a05397450978507c4ef1",
        );

        let auth = AuthScheme::digest(digest);
        assert_eq!(auth.scheme_name(), "Digest");
    }

    #[test]
    fn test_digest_builder_with_optional_fields() {
        let digest = DigestBuilder::new(
            "admin",
            "realm@example.com",
            "dcd98b7102dd2f0e8b11d0f600bfb0c093",
            "/dir/index.html",
            "6629fae49393a05397450978507c4ef1",
        )
        .algorithm("MD5")
        .cnonce("0a4f113b")
        .opaque("5ccc069c403ebaf9f0171e9517f40e41")
        .qop("auth")
        .nc("00000001");

        let auth = AuthScheme::digest(digest);
        let header = auth.to_header_value().unwrap();

        let header_str = header.to_str().unwrap();
        assert!(header_str.starts_with("Digest "));
        assert!(header_str.contains("username=\"admin\""));
        assert!(header_str.contains("realm=\"realm@example.com\""));
        assert!(header_str.contains("algorithm=MD5"));
        assert!(header_str.contains("qop=auth"));
    }

    #[test]
    fn test_digest_display() {
        let digest =
            DigestBuilder::new("testuser", "testrealm", "nonce123", "/test", "response123");

        assert_eq!(
            format!("{digest}"),
            "Digest (user: testuser, realm: testrealm)"
        );
    }

    #[test]
    fn test_digest_builder_chaining() {
        let digest = DigestBuilder::new("u", "r", "n", "uri", "resp")
            .algorithm("SHA-256")
            .qop("auth-int")
            .nc("00000002");

        let result = digest.build();
        assert!(result.contains("algorithm=SHA-256"));
        assert!(result.contains("qop=auth-int"));
        assert!(result.contains("nc=00000002"));
    }
}

mod vapid_tests {
    use asknothingx2_util::api::AuthScheme;
    use http::HeaderValue;

    #[test]
    fn test_vapid_creation() {
        let auth = AuthScheme::vapid("BNbN1RnU1Xx2dA", "mailto:admin@example.com", "signature123");

        assert_eq!(auth.scheme_name(), "VAPID");
        assert_eq!(format!("{auth}"), "VAPID (mailto:admin@example.com)");
    }

    #[test]
    fn test_vapid_header_value() {
        let auth = AuthScheme::vapid("publickey123", "mailto:test@example.com", "sig456");

        let header = auth.to_header_value().unwrap();
        let expected = "VAPID k=publickey123, a=mailto:test@example.com, s=sig456";
        assert_eq!(header, HeaderValue::from_str(expected).unwrap());
    }
}

mod scram_tests {
    use asknothingx2_util::api::{AuthScheme, SCRAMVariant};
    use http::HeaderValue;

    #[test]
    fn test_scram_sha1() {
        let auth = AuthScheme::scram(SCRAMVariant::SHA1, "credentials123");

        assert_eq!(auth.scheme_name(), "SCRAM-SHA-1");
        assert_eq!(format!("{auth}"), "SCRAM-SHA1");

        let header = auth.to_header_value().unwrap();
        assert_eq!(
            header,
            HeaderValue::from_static("SCRAM-SHA-1 credentials123")
        );
    }

    #[test]
    fn test_scram_sha256() {
        let auth = AuthScheme::scram(SCRAMVariant::SHA256, "creds456");

        assert_eq!(auth.scheme_name(), "SCRAM-SHA-256");
        assert_eq!(format!("{auth}"), "SCRAM-SHA256");

        let header = auth.to_header_value().unwrap();
        assert_eq!(header, HeaderValue::from_static("SCRAM-SHA-256 creds456"));
    }
}

mod aws4_tests {
    use asknothingx2_util::api::AuthScheme;

    #[test]
    fn test_aws4_creation() {
        let auth = AuthScheme::aws4_hmac_sha256(
            "AKIAIOSFODNN7EXAMPLE",
            "fe5a01a09a8f5db0dfe8ca4f30e11c0e4c21b93b6ff5e8b2e8d1c4e4e1d8b5c1",
            "us-east-1",
            "s3",
            "20230615T120000Z",
        );

        assert_eq!(auth.scheme_name(), "AWS4-HMAC-SHA256");
        assert_eq!(format!("{auth}"), "AWS4 (us-east-1/s3)");
    }

    #[test]
    fn test_aws4_header_value() {
        let auth = AuthScheme::aws4_hmac_sha256(
            "AKIATEST",
            "signature123",
            "us-west-2",
            "ec2",
            "20230615T120000Z",
        );

        let header = auth.to_header_value().unwrap();
        let header_str = header.to_str().unwrap();

        assert!(header_str.starts_with("AWS4-HMAC-SHA256 Credential=AKIATEST/"));
        assert!(header_str.contains("us-west-2"));
        assert!(header_str.contains("ec2"));
        assert!(header_str.contains("Signature=signature123"));
    }
}

mod other_schemes_tests {
    use asknothingx2_util::api::AuthScheme;
    use http::HeaderValue;

    #[test]
    fn test_hoba() {
        let auth = AuthScheme::hoba("result_data_here");

        assert_eq!(auth.scheme_name(), "HOBA");
        assert_eq!(format!("{auth}"), "HOBA");

        let header = auth.to_header_value().unwrap();
        assert_eq!(
            header,
            HeaderValue::from_static("HOBA result=\"result_data_here\"")
        );
    }

    #[test]
    fn test_mutual() {
        let auth = AuthScheme::mutual("mutual_credentials");

        assert_eq!(auth.scheme_name(), "Mutual");
        assert_eq!(format!("{auth}"), "Mutual");

        let header = auth.to_header_value().unwrap();
        assert_eq!(
            header,
            HeaderValue::from_static("Mutual mutual_credentials")
        );
    }

    #[test]
    fn test_negotiate() {
        let auth = AuthScheme::negotiate("negotiate_token");

        assert_eq!(auth.scheme_name(), "Negotiate");
        assert_eq!(format!("{auth}"), "Negotiate");

        let header = auth.to_header_value().unwrap();
        assert_eq!(
            header,
            HeaderValue::from_static("Negotiate negotiate_token")
        );
    }

    #[test]
    fn test_custom() {
        let auth = AuthScheme::custom("MyAuth", "custom_credentials");

        assert_eq!(auth.scheme_name(), "MyAuth");
        assert_eq!(format!("{auth}"), "Custom (MyAuth)");

        let header = auth.to_header_value().unwrap();
        assert_eq!(
            header,
            HeaderValue::from_static("MyAuth custom_credentials")
        );
    }
}

mod integration_tests {
    use asknothingx2_util::api::{AuthScheme, DigestBuilder, SCRAMVariant};

    #[test]
    fn test_auth_scheme_equality() {
        let auth1 = AuthScheme::basic("user", "pass");
        let auth2 = AuthScheme::basic("user", "pass");
        let auth3 = AuthScheme::basic("user", "different");

        assert_eq!(auth1, auth2);
        assert_ne!(auth1, auth3);
    }

    #[test]
    fn test_auth_scheme_cloning() {
        let auth = AuthScheme::bearer("token123");
        let cloned = auth.clone();

        assert_eq!(auth, cloned);
        assert_eq!(auth.scheme_name(), cloned.scheme_name());
    }

    #[test]
    fn test_auth_scheme_hashing() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let auth = AuthScheme::bearer("token");
        map.insert(auth, "bearer_auth");

        let lookup = AuthScheme::bearer("token");
        assert_eq!(map.get(&lookup), Some(&"bearer_auth"));
    }

    #[test]
    fn test_all_schemes_can_create_headers() {
        let schemes = vec![
            AuthScheme::basic("user", "pass"),
            AuthScheme::bearer("token"),
            AuthScheme::digest(DigestBuilder::new("u", "r", "n", "uri", "resp")),
            AuthScheme::hoba("result"),
            AuthScheme::mutual("creds"),
            AuthScheme::negotiate("token"),
            AuthScheme::vapid("key", "subject", "sig"),
            AuthScheme::scram(SCRAMVariant::SHA256, "creds"),
            AuthScheme::aws4_hmac_sha256("key", "sig", "region", "service", "date"),
            AuthScheme::custom("Custom", "creds"),
        ];

        for scheme in schemes {
            let result = scheme.to_header_value();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_scheme_names_are_correct() {
        let test_cases = vec![
            (AuthScheme::basic("u", "p"), "Basic"),
            (AuthScheme::bearer("t"), "Bearer"),
            (
                AuthScheme::digest(DigestBuilder::new("u", "r", "n", "uri", "resp")),
                "Digest",
            ),
            (AuthScheme::hoba("r"), "HOBA"),
            (AuthScheme::mutual("c"), "Mutual"),
            (AuthScheme::negotiate("t"), "Negotiate"),
            (AuthScheme::vapid("k", "s", "sig"), "VAPID"),
            (AuthScheme::scram(SCRAMVariant::SHA1, "c"), "SCRAM-SHA-1"),
            (
                AuthScheme::scram(SCRAMVariant::SHA256, "c"),
                "SCRAM-SHA-256",
            ),
            (
                AuthScheme::aws4_hmac_sha256("k", "s", "r", "svc", "d"),
                "AWS4-HMAC-SHA256",
            ),
            (AuthScheme::custom("MyScheme", "c"), "MyScheme"),
        ];

        for (scheme, expected_name) in test_cases {
            assert_eq!(scheme.scheme_name(), expected_name);
        }
    }
}

mod edge_cases {
    use asknothingx2_util::api::AuthScheme;

    #[test]
    fn test_extremely_long_values() {
        let long_value = "x".repeat(10000);

        let auth = AuthScheme::bearer(&long_value);
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_special_characters_in_schemes() {
        let auth = AuthScheme::custom(
            "Custom-Scheme_1.0",
            "creds-with-dashes_and_underscores.dots",
        );
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }

    #[test]
    fn test_numeric_values() {
        let auth = AuthScheme::custom("123", "456");
        let result = auth.to_header_value();
        assert!(result.is_ok());
    }
}

mod security_tests {
    use asknothingx2_util::api::AuthScheme;

    #[test]
    fn test_display_doesnt_expose_sensitive_data() {
        let auth = AuthScheme::basic("user", "secret_password");
        let display = format!("{auth}");
        assert!(!display.contains("secret_password"));
        assert!(display.contains("user"));

        let auth = AuthScheme::bearer("secret_token");
        let display = format!("{auth}");
        assert!(!display.contains("secret_token"));
    }

    #[test]
    fn test_debug_format_structure() {
        let auth = AuthScheme::basic("user", "pass");
        let debug = format!("{auth:?}");

        assert!(debug.contains("Basic"));
    }
}

mod bench_tests {
    use std::time::Instant;

    use asknothingx2_util::api::AuthScheme;

    #[test]
    fn bench_basic_auth_creation() {
        let start = Instant::now();

        for _ in 0..10000 {
            let auth = AuthScheme::basic("user", "password");
            let _ = auth.to_header_value().unwrap();
        }

        let duration = start.elapsed();
        println!("Basic auth creation: {duration:?} for 10k iterations");

        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn bench_bearer_auth_creation() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let start = Instant::now();

        for _ in 0..10000 {
            let auth = AuthScheme::bearer(token);
            let _ = auth.to_header_value().unwrap();
        }

        let duration = start.elapsed();
        println!("Bearer auth creation: {duration:?} for 10k iterations");
        assert!(duration.as_millis() < 50);
    }
}

mod property_tests {
    use asknothingx2_util::api::AuthScheme;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_basic_auth_roundtrip(
            username in "[a-zA-Z0-9_@.-]{1,100}",
            password in "[a-zA-Z0-9!@#$%^&*()_+-=]{1,100}"
        ) {
            let auth = AuthScheme::basic(&username, &password);
            let result = auth.to_header_value();

            prop_assert!(result.is_ok());

            let header = result.unwrap();
            prop_assert!(header.to_str().unwrap().starts_with("Basic "));
        }

        #[test]
        fn test_bearer_token_property(
            token in "[a-zA-Z0-9._-]{1,500}"
        ) {
            let auth = AuthScheme::bearer(&token);
            let result = auth.to_header_value();

            prop_assert!(result.is_ok());

            let header = result.unwrap();
            let expected = format!("Bearer {token}");
            prop_assert_eq!(header.to_str().unwrap(), expected);
        }
    }
}
