use http::{HeaderMap, HeaderValue, StatusCode};

#[cfg(feature = "oauth")]
pub use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RefreshToken, RevocationUrl, Scope, TokenUrl,
};

#[cfg(feature = "api-request")]
#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

#[cfg(feature = "api-request")]
#[allow(non_snake_case)]
pub fn CONTENT_TYPE_JSON() -> HeaderValue {
    HeaderValue::from_str("application/json").unwrap()
}

#[cfg(feature = "api-request")]
#[allow(non_snake_case)]
pub fn CONTENT_TYPE_FORMENCODED() -> HeaderValue {
    HeaderValue::from_str("application/x-www-form-urlencoded").unwrap()
}

#[cfg(test)]
mod tqpe_tests {
    use crate::tqpe::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON};

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
