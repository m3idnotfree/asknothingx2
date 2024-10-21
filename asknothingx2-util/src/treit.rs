#[cfg(feature = "api-request")]
pub trait APIRequest {
    fn method(&self) -> http::Method;
    fn url(&self) -> url::Url;
    fn headers(&self) -> http::HeaderMap {
        http::HeaderMap::new()
    }
    fn json(&self) -> Option<String> {
        None
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        None
    }

    fn form_urlencoded_serializere_pairs(params: Vec<(&str, &str)>) -> Vec<u8> {
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes()
    }
}
