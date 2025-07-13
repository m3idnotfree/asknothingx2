use http::{
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CACHE_CONTROL,
        CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT,
    },
    HeaderMap, HeaderName, HeaderValue,
};

use crate::api::{
    mime_type::{Application, Multipart, Text},
    AuthScheme,
};

use super::{error, Error};

pub mod headers {
    use http::HeaderName;

    pub const CLIENT_ID: HeaderName = HeaderName::from_static("client-id");
    pub const X_API_KEY: HeaderName = HeaderName::from_static("x-api-key");
    pub const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");
}

pub struct HeaderMut<'a> {
    header: &'a mut HeaderMap,
}

impl<'a> HeaderMut<'a> {
    pub fn new(header: &'a mut HeaderMap) -> Self {
        Self { header }
    }

    pub fn header(&mut self, key: HeaderName, value: HeaderValue) -> &mut Self {
        self.header.insert(key, value);
        self
    }

    pub fn header_static(&mut self, key: HeaderName, value: &'static str) -> &mut Self {
        self.header.insert(key, HeaderValue::from_static(value));
        self
    }

    pub fn header_str(&mut self, key: HeaderName, value: &str) -> Result<&mut Self, Error> {
        let val = HeaderValue::from_str(value).map_err(error::http::invalid_header)?;

        self.header.insert(key, val);
        Ok(self)
    }

    pub fn append(&mut self, key: HeaderName, value: HeaderValue) -> &mut Self {
        self.header.append(key, value);

        self
    }

    pub fn extend(&mut self, headers: HeaderMap) -> &mut Self {
        self.header.extend(headers);

        self
    }

    pub fn is_empty(&self) -> bool {
        self.header.is_empty()
    }

    /// Client-Id: id
    pub fn client_id(&mut self, id: &str) -> Result<&mut Self, Error> {
        self.header_str(headers::CLIENT_ID, id)
    }

    /// User-Agent: agent
    pub fn user_agent(&mut self, agent: &str) -> Result<&mut Self, Error> {
        self.header_str(USER_AGENT, agent)
    }

    /// Cache-Control: no-cache
    pub fn cache_control_no_cache(&mut self) -> &mut Self {
        self.header
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        self
    }

    /// Cache-Control: value
    pub fn cache_control(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header_str(CACHE_CONTROL, value)
    }

    /// X-API-Key: key
    pub fn api_key(&mut self, key: &str) -> Result<&mut Self, Error> {
        self.header_str(headers::X_API_KEY, key)
    }

    /// X-Request-ID: id
    pub fn request_id(&mut self, id: &str) -> Result<&mut Self, Error> {
        self.header_str(headers::X_REQUEST_ID, id)
    }

    /// Origin: origin
    pub fn origin(&mut self, origin: &str) -> Result<&mut Self, Error> {
        self.header_str(ORIGIN, origin)
    }

    /// Referer: referer
    pub fn referer(&mut self, referer: &str) -> Result<&mut Self, Error> {
        self.header_str(REFERER, referer)
    }

    // CORS headers
    /// Access-Control-Allow-Origin: *
    pub fn cors_allow_all(&mut self) -> &mut Self {
        self.header
            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        self
    }

    /// Access-Control-Allow-Origin: origin
    pub fn cors_allow_origin(&mut self, origin: &str) -> Result<&mut Self, Error> {
        self.header_str(ACCESS_CONTROL_ALLOW_ORIGIN, origin)
    }

    /// Access-Control-Allow-Methods: GET, POST, PUT, DELETE
    pub fn cors_allow_methods_standard(&mut self) -> &mut Self {
        self.header.insert(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET,POST,PUT,DELETE"),
        );
        self
    }

    /// Access-Control-Allow-Headers: Content-Type, Authorization
    pub fn cors_allow_headers_standard(&mut self) -> &mut Self {
        self.header.insert(
            ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("Content-Type,Authorization"),
        );
        self
    }

    /// Connection: keep-alive
    pub fn connection_keep_alive(&mut self) -> &mut Self {
        self.header
            .insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        self
    }

    /// Connection: close
    pub fn connection_close(&mut self) -> &mut Self {
        self.header
            .insert(CONNECTION, HeaderValue::from_static("close"));
        self
    }

    /// Content-Length: <length>
    pub fn content_length(&mut self, length: u64) -> &mut Self {
        self.header.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&length.to_string()).unwrap(),
        );
        self
    }

    /// Accept: application/json, Content-Type: application/json
    pub fn json_api(&mut self) -> &mut Self {
        self.accept_json().content_type_json()
    }
}

impl<'a> HeaderMut<'a> {
    /// ACCEPT: application/json
    pub fn accept_json(&mut self) -> &mut Self {
        self.header
            .insert(ACCEPT, Application::Json.to_header_value());
        self
    }

    /// ACCEPT: text/html
    pub fn accept_html(&mut self) -> &mut Self {
        self.header.insert(ACCEPT, Text::Html.to_header_value());
        self
    }

    /// ACCEPT: text/plain
    pub fn accept_text(&mut self) -> &mut Self {
        self.header
            .insert(ACCEPT, HeaderValue::from_static("text/plain"));
        self
    }

    /// ACCEPT: */*
    pub fn accept_any(&mut self) -> &mut Self {
        self.header.insert(ACCEPT, HeaderValue::from_static("*/*"));
        self
    }

    /// ACCEPT: multi items
    pub fn accept_mulity(&mut self, items: &[&str]) -> Result<&mut Self, Error> {
        self.header_str(ACCEPT, &items.join(","))
    }

    /// Accept-Encoding: gzip, deflate, br
    pub fn accept_encoding_standard(&mut self) -> &mut Self {
        self.header
            .insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip,deflate,br"));
        self
    }

    /// Accept-Language: en-US, en;q=0.9
    pub fn accept_language_en(&mut self) -> &mut Self {
        self.header
            .insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        self
    }

    /// Accept-Language: lang
    pub fn accept_language(&mut self, lang: &str) -> Result<&mut Self, Error> {
        self.header_str(ACCEPT_LANGUAGE, lang)
    }
}

impl<'a> HeaderMut<'a> {
    /// CONTENT-TYPE: application/x-www-form-urlencoded
    pub fn content_type_formencoded(&mut self) -> &mut Self {
        self.header
            .insert(CONTENT_TYPE, Application::FormUrlEncoded.to_header_value());
        self
    }

    /// CONTENT-TYPE: application/json
    pub fn content_type_json(&mut self) -> &mut Self {
        self.header
            .insert(CONTENT_TYPE, Application::Json.to_header_value());
        self
    }

    /// CONTENT-TYPE: text/plain
    pub fn content_type_text(&mut self) -> &mut Self {
        self.header
            .insert(CONTENT_TYPE, Text::Plain.to_header_value());
        self
    }

    /// CONTENT-TYPE: text/html
    pub fn content_type_html(&mut self) -> &mut Self {
        self.header
            .insert(CONTENT_TYPE, Text::Html.to_header_value());
        self
    }

    /// CONTENT-TYPE: multipart/form-data
    pub fn content_type_multipart(&mut self) -> &mut Self {
        self.header
            .insert(CONTENT_TYPE, Multipart::FormData.to_header_value());
        self
    }
}

impl<'a> HeaderMut<'a> {
    /// Authorization: <type> <credentials>
    pub fn authorization(&mut self, auth: AuthScheme) -> &mut Self {
        self.header
            .insert(AUTHORIZATION, auth.to_header_value().unwrap());
        self
    }

    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.authorization(AuthScheme::basic(username, password))
    }

    pub fn bearer_token(&mut self, token: &str) -> &mut Self {
        self.authorization(AuthScheme::bearer(token))
    }
}
