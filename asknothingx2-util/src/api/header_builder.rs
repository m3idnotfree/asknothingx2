use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderMap,
};

use super::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON};

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
    pub fn accept_json(mut self) -> Self {
        self._inner.append(ACCEPT, CONTENT_TYPE_JSON());
        self
    }

    /// CONTENT-TYPE: application/x-www-form-urlencoded
    pub fn content_type_formencoded(mut self) -> Self {
        self._inner.append(CONTENT_TYPE, CONTENT_TYPE_FORMENCODED());
        self
    }

    pub fn build(self) -> HeaderMap {
        self._inner
    }
}
