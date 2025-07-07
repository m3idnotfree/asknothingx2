use asknothingx2_util::api::request::RequestBody;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_string_body_roundtrip(s in ".*") {
        let body = RequestBody::from_string(s.clone());
        if s.is_empty() {
            prop_assert!(body.is_empty());
        } else {
            prop_assert!(!body.is_empty());
        }
        prop_assert_eq!(body.content_length(), Some(s.len() as u64));
    }

    #[test]
    fn test_bytes_body_consistency(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let body = RequestBody::from_vec(data.clone());
        prop_assert_eq!(body.content_length(), Some(data.len() as u64));
        prop_assert_eq!(body.is_empty(), data.is_empty());
    }

    #[test]
    fn test_form_body_consistency(
        keys in prop::collection::vec("[a-zA-Z0-9_]+", 0..10),
        values in prop::collection::vec(".*", 0..10)
    ) {
        let min_len = keys.len().min(values.len());
        let form_data: Vec<(String, String)> = keys.into_iter()
            .zip(values.into_iter())
            .take(min_len)
            .collect();

        let body = RequestBody::from_form(form_data.clone());
        prop_assert_eq!(body.is_empty(), form_data.is_empty());

        if !form_data.is_empty() {
            prop_assert!(body.content_length().is_some());
            prop_assert!(body.content_length().unwrap() > 0);
        }
    }
}

#[test]
fn test_known_static_strings() {
    let test_strings = [
        "",
        "hello",
        "hello world",
        "special chars: !@#$%^&*()",
        "unicode: 你好世界 🌍",
        "numbers: 1234567890",
    ];

    for &s in &test_strings {
        let leaked_str: &'static str = Box::leak(s.to_string().into_boxed_str());
        let body = RequestBody::from_static(leaked_str);
        assert_eq!(body.is_empty(), s.is_empty());
        assert_eq!(body.content_length(), Some(s.len() as u64));
    }
}
