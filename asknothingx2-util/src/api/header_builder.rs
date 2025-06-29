use std::str::FromStr;

use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderMap, HeaderName, HeaderValue,
};

use super::ContentType;

#[derive(Debug)]
pub struct HeaderBuilder {
    _inner: HeaderMap,
}

#[allow(clippy::new_without_default)]
impl HeaderBuilder {
    pub fn new() -> Self {
        Self {
            _inner: HeaderMap::new(),
        }
    }

    /// ACCEPT: application/json
    pub fn accept_json(&mut self) -> &mut Self {
        self._inner
            .insert(ACCEPT, ContentType::Json.as_header_value());
        self
    }

    /// CONTENT-TYPE: application/x-www-form-urlencoded
    pub fn content_type_formencoded(&mut self) -> &mut Self {
        self._inner
            .insert(CONTENT_TYPE, ContentType::FormEncoded.as_header_value());
        self
    }

    /// CONTENT-TYPE: application/json
    pub fn content_type_json(&mut self) -> &mut Self {
        self._inner
            .insert(CONTENT_TYPE, ContentType::Json.as_header_value());
        self
    }

    pub fn append(&mut self, key: &str, value: &str) -> Result<&mut Self, http::Error> {
        self._inner
            .append(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);

        Ok(self)
    }

    /// Authorization: <type> <credentials>
    pub fn authorization(&mut self, kind: &str, credentials: &str) -> &mut Self {
        self._inner.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("{} {}", kind, credentials)).unwrap(),
        );
        self
    }

    /// Client-Id: <id>
    pub fn client_id(&mut self, id: &str) -> &mut Self {
        self._inner.insert(
            HeaderName::from_str("Client-Id").unwrap(),
            HeaderValue::from_str(id).unwrap(),
        );

        self
    }

    pub fn build(self) -> HeaderMap {
        self._inner
    }
}

#[cfg(test)]
mod test {
    use http::HeaderValue;

    use super::HeaderBuilder;

    #[test]
    fn header_builder() {
        let headers = HeaderBuilder::new().build();
        assert_eq!(0, headers.len());

        let mut headers = HeaderBuilder::new();
        headers.content_type_formencoded();
        let headers = headers.build();
        assert_eq!(1, headers.len());
        let content_type = headers.get("content-type");
        assert!(content_type.is_some());
        let content_type = headers.get("CONTENT-TYPE");
        assert!(content_type.is_some());

        assert_eq!(
            Some(HeaderValue::from_str("application/x-www-form-urlencoded").unwrap()),
            content_type.cloned()
        );
        assert_ne!(
            Some(HeaderValue::from_str("APPLICATION/X-WWW-FORM-URLENCODED").unwrap()),
            content_type.cloned()
        );
        let mut headers = HeaderBuilder::new();
        headers.content_type_formencoded().accept_json();

        let headers = headers.build();
        assert_eq!(2, headers.len());

        let accept = headers.get("accept");
        assert!(accept.is_some());
        let accept = headers.get("ACCEPT");
        assert!(accept.is_some());
        assert_eq!(
            Some(HeaderValue::from_str("application/json").unwrap()),
            accept.cloned()
        );

        let mut headers = HeaderBuilder::new();
        headers
            .content_type_formencoded()
            .accept_json()
            .authorization("Oauth", "credentials");
        let headers = headers.build();
        assert_eq!(3, headers.len());
        let authorization = headers.get("authorization");
        assert!(authorization.is_some());
        let authorization = headers.get("AUTHORIZATION");
        assert!(authorization.is_some());

        assert_eq!(
            Some(HeaderValue::from_str("Oauth credentials").unwrap()),
            authorization.cloned()
        );
    }
}
