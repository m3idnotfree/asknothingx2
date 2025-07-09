mod body;
mod error;

#[cfg(feature = "stream")]
pub use body::CodecType;

pub use body::RequestBody;
pub use error::{
    FileOperation, HeaderError, LimitType, NetworkOperation, ProcessOperation, StreamError,
    StreamOperation,
};

use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};

use std::str::FromStr;

use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Client, Request, RequestBuilder, Response};
use url::Url;

use super::{
    content_type::{Application, Text},
    setup::get_global_client_or_default,
    HeaderMut,
};

/// Characters that must be percent-encoded in HTTP header values
/// Based on RFC 7230 field-vchar = VCHAR / obs-text
const HEADER_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'\\')
    .add(b'\t')
    .add(b'\r')
    .add(b'\n');

/// Stricter encoding for header values that should be safe everywhere
const HEADER_SAFE_ENCODE_SET: &AsciiSet = &HEADER_ENCODE_SET
    .add(b'(')
    .add(b')')
    .add(b'<')
    .add(b'>')
    .add(b'@')
    .add(b',')
    .add(b';')
    .add(b':')
    .add(b'/')
    .add(b'[')
    .add(b']')
    .add(b'?')
    .add(b'=')
    .add(b'{')
    .add(b'}');

/// For RFC 8187 filename* encoding - encode control chars and non-ASCII
/// RFC 8187 allows most ASCII chars but requires encoding of control chars
const RFC8187_ENCODE_SET: &AsciiSet = CONTROLS;

/// Alternative: More restrictive set for RFC 8187 if needed
/// This encodes additional characters that might cause issues in filenames
const RFC8187_SAFE_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'%')
    .add(b'*')
    .add(b'/')
    .add(b'\\')
    .add(b'?')
    .add(b'<')
    .add(b'>')
    .add(b'|');

pub trait IntoRequestParts {
    fn into_request_parts(self) -> RequestParts;
}

#[derive(Debug)]
pub struct RequestParts {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub body: Option<RequestBody>,
    pub version: Option<http::Version>,
    pub timeout: Option<std::time::Duration>,
    pub request_id: Option<String>,
}

impl RequestParts {
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: HeaderMap::new(),
            body: None,
            version: None,
            timeout: None,
            request_id: None,
        }
    }

    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_str(key.as_ref()),
            HeaderValue::from_str(value.as_ref()),
        ) {
            self.headers.insert(name, val);
        }
        self
    }

    pub fn try_header(
        mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self, HeaderError> {
        let key_str = key.as_ref();
        let value_str = value.as_ref();

        let name = HeaderName::from_str(key_str).map_err(|e| HeaderError::InvalidHeaderName {
            name: key_str.to_string(),
            reason: e.to_string(),
        })?;

        let val =
            HeaderValue::from_str(value_str).map_err(|e| HeaderError::InvalidHeaderValue {
                name: key_str.to_string(),
                value: value_str.to_string(),
                reason: e.to_string(),
            })?;

        self.headers.insert(name, val);
        Ok(self)
    }

    // pub fn header_encoded(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    //     let key_str = key.as_ref();
    //     let value_str = value.as_ref();
    //
    //     let name = match HeaderName::from_str(key_str) {
    //         Ok(name) => name,
    //         Err(e) => {
    //             warn!(
    //                 header_name = key_str,
    //                 error =%e,
    //                 "Invalid header name"
    //             );
    //             return self;
    //         }
    //     };
    //
    //     if let Ok(val) = HeaderValue::from_str(value_str) {
    //         self.headers.insert(name, val);
    //         return self;
    //     }
    //
    //     let encoded_value = utf8_percent_encode(value_str, HEADER_ENCODE_SET).to_string();
    //     if let Ok(val) = HeaderValue::from_str(&encoded_value) {
    //         debug!(
    //             header_name = key_str,
    //             original_value = value_str,
    //             encoded_value = %encoded_value,
    //             "Header value percent-encoded"
    //         );
    //         self.headers.insert(name, val);
    //     } else {
    //         warn!(
    //             header_name = key_str,
    //             header_value = value_str,
    //             encoded_value = %encoded_value,
    //             "Could not set header even with percent encoding"
    //         );
    //     }
    //
    //     self
    // }

    // pub fn header_rfc8187(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    //     let key_str = key.as_ref();
    //     let value_str = value.as_ref();
    //
    //     let name = match HeaderName::from_str(key_str) {
    //         Ok(name) => name,
    //         Err(e) => {
    //             warn!(
    //                 header_name = key_str,
    //                 error = %e,
    //                 "Invalid header name for RFC 8187 encoding"
    //             );
    //             return self;
    //         }
    //     };
    //
    //     let encoded_value = if value_str.is_ascii() {
    //         value_str.to_string()
    //     } else {
    //         let percent_encoded = utf8_percent_encode(value_str, RFC8187_ENCODE_SET).to_string();
    //         let rfc8187_value = format!("utf-8''{percent_encoded}");
    //         debug!(
    //             header_name = key_str,
    //             original_value = value_str,
    //             rfc8187_value = %rfc8187_value,
    //             "Applied RFC 8187 encoding to header value"
    //         );
    //         rfc8187_value
    //     };
    //
    //     if let Ok(val) = HeaderValue::from_str(&encoded_value) {
    //         self.headers.insert(name, val);
    //     } else {
    //         warn!(
    //             header_name = key_str,
    //             original_value = value_str,
    //             encoded_value = %encoded_value,
    //             "Could not set header with RFC 8187 encoding"
    //         );
    //     }
    //
    //     self
    // }

    // Set Content-Disposition with proper filename encoding
    //
    // This is a convenience method that handles both `filename` and `filename*`
    // parameters according to RFC 6266 for maximum browser compatibility.
    // pub fn content_disposition_attachment(mut self, filename: impl AsRef<str>) -> Self {
    //     let filename_str = filename.as_ref();
    //
    //     // Create ASCII fallback by replacing non-ASCII chars
    //     let ascii_fallback = filename_str
    //         .chars()
    //         .map(|c| {
    //             if c.is_ascii() && c != '"' && c != '\\' {
    //                 c
    //             } else {
    //                 '_'
    //             }
    //         })
    //         .collect::<String>();
    //
    //     let disposition_value = if filename_str.is_ascii() {
    //         // Simple case: pure ASCII filename
    //         debug!(
    //             filename = filename_str,
    //             "Setting ASCII filename in Content-Disposition"
    //         );
    //
    //         format!("attachment; filename=\"{filename_str}\"")
    //     } else {
    //         // Complex case: provide both filename and filename* for compatibility
    //         let encoded_filename =
    //             utf8_percent_encode(filename_str, RFC8187_ENCODE_SET).to_string();
    //         debug!(
    //             filename = filename_str,
    //             ascii_fallback = %ascii_fallback,
    //             encoded_filename = %encoded_filename,
    //             "Setting international filename with RFC 8187 encoding"
    //         );
    //
    //         format!(
    //             "attachment; filename=\"{ascii_fallback}\"; filename*=utf-8''{encoded_filename}",
    //         )
    //     };
    //
    //     self.header("Content-Disposition", disposition_value)
    // }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn header_mut(&mut self) -> HeaderMut<'_> {
        HeaderMut::new(&mut self.headers)
    }

    pub fn body(mut self, body: RequestBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = Some(RequestBody::from_string(text.into()));
        self.header(CONTENT_TYPE, Text::Plain)
    }

    pub fn json(mut self, value: serde_json::Value) -> Self {
        self.body = Some(RequestBody::from_json(value));
        self.header(CONTENT_TYPE, Application::Json)
    }

    pub fn form(mut self, form: Vec<(String, String)>) -> Self {
        self.body = Some(RequestBody::from_form(form));
        self.header(CONTENT_TYPE, Application::FormUrlEncoded)
    }

    pub fn form_pairs<I, K, V>(mut self, pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.body = Some(RequestBody::from_form_pairs(pairs));
        self.header(CONTENT_TYPE, Application::FormUrlEncoded)
    }

    pub fn multipart(mut self, form: reqwest::multipart::Form) -> Self {
        self.body = Some(RequestBody::from_multipart(form));
        // Note: reqwest will set the content-type with boundary automatically
        self
    }

    pub fn empty(mut self) -> Self {
        self.body = Some(RequestBody::empty());
        self
    }

    pub fn version(mut self, version: http::Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn into_request_builder(self, client: &Client) -> (RequestBuilder, Option<String>) {
        let mut builder = client.request(self.method, self.url);

        if !self.headers.is_empty() {
            builder = builder.headers(self.headers);
        }

        if let Some(version) = self.version {
            builder = builder.version(version);
        }

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(body) = self.body {
            builder = body.into_reqwest_body(builder);
        }

        (builder, self.request_id)
    }

    pub fn into_request(self, client: &Client) -> Result<Request, reqwest::Error> {
        let (request_builder, _) = self.into_request_builder(client);
        request_builder.build()
    }

    pub async fn send(self) -> Result<Response, reqwest::Error> {
        let (request_builder, _) = self.into_request_builder(get_global_client_or_default());
        request_builder.send().await
    }

    #[cfg(feature = "stream")]
    pub fn from_file(mut self, file: tokio::fs::File) -> Self {
        self.body = Some(RequestBody::from_file(file));
        self
    }

    #[cfg(feature = "stream")]
    pub fn from_file_buffered(mut self, file: tokio::fs::File, buffer_size: usize) -> Self {
        self.body = Some(RequestBody::from_file_buffered(file, buffer_size));
        self
    }

    #[cfg(feature = "stream")]
    pub async fn from_file_path<P: AsRef<std::path::Path>>(
        mut self,
        path: P,
    ) -> Result<Self, StreamError> {
        self.body = Some(RequestBody::from_file_path(path).await?);
        Ok(self)
    }

    #[cfg(feature = "stream")]
    pub async fn from_file_path_buffered<P: AsRef<std::path::Path>>(
        mut self,
        path: P,
        buffer_size: usize,
    ) -> Result<Self, StreamError> {
        self.body = Some(RequestBody::from_file_path_buffered(path, buffer_size).await?);
        Ok(self)
    }

    #[cfg(feature = "stream")]
    pub fn from_async_read<R>(mut self, reader: R) -> Self
    where
        R: tokio::io::AsyncRead + Send + Sync + 'static,
    {
        self.body = Some(RequestBody::from_async_read(reader));
        self
    }

    #[cfg(feature = "stream")]
    pub fn from_tcp_stream(mut self, tcp: tokio::net::TcpStream) -> Self {
        self.body = Some(RequestBody::from_tcp_stream(tcp));
        self
    }

    #[cfg(feature = "stream")]
    pub fn from_command_output(
        mut self,
        command: tokio::process::Command,
    ) -> Result<Self, StreamError> {
        self.body = Some(RequestBody::from_command_output(command)?);
        Ok(self)
    }

    #[cfg(feature = "stream")]
    pub fn stream<S>(mut self, stream: S) -> Self
    where
        S: futures_util::Stream<Item = Result<bytes::Bytes, StreamError>> + Send + Sync + 'static,
    {
        self.body = Some(RequestBody::from_stream(stream));
        self
    }

    #[cfg(feature = "stream")]
    pub fn io_stream<S>(mut self, stream: S) -> Self
    where
        S: futures_util::Stream<Item = Result<bytes::Bytes, std::io::Error>>
            + Send
            + Sync
            + 'static,
    {
        self.body = Some(RequestBody::from_io_stream(stream));
        self
    }
}

impl IntoRequestParts for RequestParts {
    fn into_request_parts(self) -> RequestParts {
        self
    }
}

/// Decode a percent-encoded header value back to UTF-8
pub fn decode_header_value(encoded: &str) -> Result<String, HeaderError> {
    percent_decode_str(encoded)
        .decode_utf8()
        .map(|cow| cow.into_owned())
        .map_err(|e| HeaderError::InvalidUtf8 {
            reason: e.to_string(),
        })
}

/// Encode a string using the safe header encoding set
pub fn encode_header(value: &str) -> String {
    utf8_percent_encode(value, HEADER_SAFE_ENCODE_SET).to_string()
}

/// Create an RFC 8187 encoded value
pub fn encode_rfc8187(value: &str) -> String {
    if value.is_ascii() {
        value.to_string()
    } else {
        let encoded = utf8_percent_encode(value, RFC8187_ENCODE_SET).to_string();
        format!("utf-8''{encoded}")
    }
}
