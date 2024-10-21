use http::HeaderValue;

#[allow(non_snake_case)]
pub fn CONTENT_TYPE_JSON() -> HeaderValue {
    HeaderValue::from_str("application/json").unwrap()
}

#[allow(non_snake_case)]
pub fn CONTENT_TYPE_FORMENCODED() -> HeaderValue {
    HeaderValue::from_str("application/x-www-form-urlencoded").unwrap()
}

#[allow(non_snake_case)]
pub fn CONTENT_TYPE_TEXT() -> HeaderValue {
    HeaderValue::from_str("text/plain").unwrap()
}

#[cfg(test)]
mod tqpe_tests {
    use crate::api::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON, CONTENT_TYPE_TEXT};

    #[test]
    fn content_type_json() {
        assert_eq!(CONTENT_TYPE_JSON(), "application/json");
    }

    #[test]
    fn content_type_formencoded() {
        assert_eq!(
            CONTENT_TYPE_FORMENCODED(),
            "application/x-www-form-urlencoded"
        );
    }

    #[test]
    fn content_type_text() {
        assert_eq!(CONTENT_TYPE_TEXT(), "text/plain");
    }
}
