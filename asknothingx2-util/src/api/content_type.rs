use http::HeaderValue;

#[allow(non_snake_case)]
pub fn CONTENT_TYPE_JSON() -> HeaderValue {
    HeaderValue::from_str("application/json").unwrap()
}

#[allow(non_snake_case)]
pub fn CONTENT_TYPE_FORMENCODED() -> HeaderValue {
    HeaderValue::from_str("application/x-www-form-urlencoded").unwrap()
}

#[cfg(test)]
mod tqpe_tests {
    use crate::api::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON};

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
}
