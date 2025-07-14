#[macro_use]
mod macros;

mod application;
mod audio;
mod chemical;
mod font;
mod image;
mod message;
mod model;
mod multipart;
mod text;
mod video;

pub use application::Application;
pub use audio::Audio;
pub use chemical::Chemical;
pub use error::Error;
pub use font::Font;
pub use image::Image;
pub use message::Message;
pub use model::Model;
pub use multipart::Multipart;
pub use text::Text;
pub use video::Video;

use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};

use super::error;

#[derive(Debug, Clone, Eq)]
pub enum MimeType {
    Application(Application),
    Audio(Audio),
    Chemical(Chemical),
    Font(Font),
    Image(Image),
    Message(Message),
    Model(Model),
    Multipart(Multipart),
    Text(Text),
    Video(Video),
    Custom(String),
}

impl MimeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Application(app) => app.as_str(),
            Self::Audio(audio) => audio.as_str(),
            Self::Chemical(chemical) => chemical.as_str(),
            Self::Font(font) => font.as_str(),
            Self::Image(image) => image.as_str(),
            Self::Message(message) => message.as_str(),
            Self::Model(model) => model.as_str(),
            Self::Multipart(multipart) => multipart.as_str(),
            Self::Text(text) => text.as_str(),
            Self::Video(video) => video.as_str(),
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn as_header_value(&self) -> HeaderValue {
        match self {
            Self::Application(s) => s.as_header_value(),
            Self::Audio(s) => s.as_header_value(),
            Self::Chemical(s) => s.as_header_value(),
            Self::Font(s) => s.as_header_value(),
            Self::Image(s) => s.as_header_value(),
            Self::Message(s) => s.as_header_value(),
            Self::Model(s) => s.as_header_value(),
            Self::Multipart(s) => s.as_header_value(),
            Self::Text(s) => s.as_header_value(),
            Self::Video(s) => s.as_header_value(),
            Self::Custom(s) => HeaderValue::from_str(s.as_str()).unwrap(),
        }
    }

    pub fn to_header_value(self) -> HeaderValue {
        match self {
            Self::Application(s) => s.to_header_value(),
            Self::Audio(s) => s.to_header_value(),
            Self::Chemical(s) => s.to_header_value(),
            Self::Font(s) => s.to_header_value(),
            Self::Image(s) => s.to_header_value(),
            Self::Message(s) => s.to_header_value(),
            Self::Model(s) => s.to_header_value(),
            Self::Multipart(s) => s.to_header_value(),
            Self::Text(s) => s.to_header_value(),
            Self::Video(s) => s.to_header_value(),
            Self::Custom(s) => HeaderValue::from_str(s.as_str()).unwrap(),
        }
    }

    pub fn from_header_value(value: &HeaderValue) -> Result<Self, Error> {
        let content_type = value
            .to_str()
            .map_err(|_| error::content::invalid_type("invalid UTF-8 in content type header"))?;

        Self::from_str(content_type)
    }

    pub fn extract_charset(content_type: &str) -> Option<&str> {
        ParsedMimeType::parse_str(content_type).ok()?.charset()
    }

    pub fn extract_boundary(content_type: &str) -> Option<&str> {
        ParsedMimeType::parse_str(content_type).ok()?.boundary()
    }

    pub fn matches_with_params(content_type: &str, expected: Self) -> bool {
        Self::from_str(content_type)
            .map(|parsed| parsed == expected)
            .unwrap_or(false)
    }

    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub const fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    pub const fn is_media(&self) -> bool {
        matches!(self, Self::Audio(_) | Self::Video(_))
    }

    pub const fn is_multipart(&self) -> bool {
        matches!(self, Self::Multipart(_))
    }

    pub fn set_on_headers(self, headers: &mut HeaderMap) {
        headers.insert(CONTENT_TYPE, self.to_header_value());
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for MimeType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = ParsedMimeType::parse_str(s)?;

        let mime_type = parsed.mime_type;

        if let Ok(s) = Text::from_str(s) {
            return Ok(MimeType::Text(s));
        }

        if let Ok(s) = Application::from_str(mime_type) {
            return Ok(MimeType::Application(s));
        }

        if let Ok(s) = Image::from_str(s) {
            return Ok(MimeType::Image(s));
        }

        if let Ok(s) = Video::from_str(s) {
            return Ok(MimeType::Video(s));
        }

        if let Ok(s) = Audio::from_str(s) {
            return Ok(MimeType::Audio(s));
        }

        if let Ok(s) = Font::from_str(s) {
            return Ok(MimeType::Font(s));
        }

        if let Ok(s) = Model::from_str(s) {
            return Ok(MimeType::Model(s));
        }

        if let Ok(s) = Chemical::from_str(s) {
            return Ok(MimeType::Chemical(s));
        }

        if let Ok(s) = Message::from_str(s) {
            return Ok(MimeType::Message(s));
        }

        if let Ok(s) = Multipart::from_str(s) {
            return Ok(MimeType::Multipart(s));
        }

        Ok(MimeType::Custom(s.to_string()))
    }
}

impl TryFrom<&str> for MimeType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for MimeType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&HeaderValue> for MimeType {
    type Error = Error;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        Self::from_header_value(value)
    }
}

impl From<MimeType> for String {
    fn from(value: MimeType) -> Self {
        value.to_string()
    }
}

impl From<MimeType> for HeaderValue {
    fn from(value: MimeType) -> Self {
        value.to_header_value()
    }
}

impl PartialEq for MimeType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
}

impl PartialEq<String> for MimeType {
    fn eq(&self, other: &String) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<MimeType> for String {
    fn eq(&self, other: &MimeType) -> bool {
        self.eq_ignore_ascii_case(other.as_str())
    }
}

impl PartialEq<&str> for MimeType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<MimeType> for &str {
    fn eq(&self, other: &MimeType) -> bool {
        self.eq_ignore_ascii_case(other.as_str())
    }
}

impl PartialEq<HeaderValue> for MimeType {
    fn eq(&self, other: &HeaderValue) -> bool {
        other.eq(self.as_str())
    }
}

impl PartialEq<MimeType> for HeaderValue {
    fn eq(&self, other: &MimeType) -> bool {
        self.eq(other.as_str())
    }
}

impl Hash for MimeType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().to_lowercase().hash(state)
    }
}

#[derive(Debug, Clone)]
pub struct ParsedMimeType<'a> {
    mime_type: &'a str,
    parameters: &'a str,
    raw: &'a str,
}

impl<'a> ParsedMimeType<'a> {
    pub fn as_str(&self) -> &'a str {
        self.raw
    }

    pub fn mime_type(&self) -> Result<MimeType, Error> {
        MimeType::from_str(self.mime_type)
    }

    pub fn raw_mime_type(&self) -> &'a str {
        self.mime_type
    }

    pub fn parse(header_value: &'a HeaderValue) -> Result<Self, Error> {
        let content_type_str = header_value
            .to_str()
            .map_err(|_| error::content::invalid_type("invalid UTF-8 in content type header"))?;

        Self::parse_str(content_type_str)
    }

    pub fn parse_str(input: &'a str) -> Result<Self, Error> {
        let input = input.trim();

        if input.is_empty() {
            return Err(error::content::invalid_type("empty content type"));
        }

        if input.len() > 1000 {
            return Err(error::content::invalid_type("content type too long"));
        }

        if let Some(semicolon_pos) = input.find(';') {
            let mime_type = input[..semicolon_pos].trim();
            let parameters = input[semicolon_pos + 1..].trim();

            if !Self::is_valid_mime_type(mime_type) {
                return Err(error::content::invalid_type(format!(
                    "invalid MIME type: {mime_type}"
                )));
            }

            Ok(Self {
                mime_type,
                parameters,
                raw: input,
            })
        } else {
            if !Self::is_valid_mime_type(input) {
                return Err(error::content::invalid_type(format!(
                    "invalid MIME type: {input}"
                )));
            }

            Ok(Self {
                mime_type: input,
                parameters: "",
                raw: input,
            })
        }
    }

    fn is_valid_mime_type(mime_type: &str) -> bool {
        if mime_type.is_empty() || mime_type.len() > 200 {
            return false;
        }

        if !mime_type.is_ascii() {
            return false;
        }

        let bytes = mime_type.as_bytes();
        let mut slash_pos = None;

        for (i, &byte) in bytes.iter().enumerate() {
            if !is_valid_mime_token_byte(byte) {
                return false;
            }

            if byte == b'/' {
                if slash_pos.is_some() {
                    return false;
                }
                slash_pos = Some(i);
            }
        }

        matches!(slash_pos, Some(pos) if pos > 0 && pos < bytes.len() - 1)
    }

    pub fn get(&self, key: &str) -> Option<&'a str> {
        for param in self.parameters.split(';') {
            if let Some(eq_pos) = param.find('=') {
                let param_key = param[..eq_pos].trim();
                if param_key.eq_ignore_ascii_case(key) {
                    return Some(Self::unquote(param[eq_pos + 1..].trim()));
                }
            }
        }
        None
    }

    fn unquote(value: &str) -> &str {
        if value.len() >= 2
            && ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
        {
            return &value[1..value.len() - 1];
        }
        value
    }

    pub fn charset(&self) -> Option<&'a str> {
        self.get("charset")
    }

    pub fn boundary(&self) -> Option<&'a str> {
        self.get("boundary")
    }

    pub fn version(&self) -> Option<&'a str> {
        self.get("version")
    }

    pub fn profile(&self) -> Option<&'a str> {
        self.get("profile")
    }

    pub fn parameter_count(&self) -> usize {
        if self.parameters.is_empty() {
            return 0;
        }

        self.parameters
            .split(';')
            .filter(|p| !p.trim().is_empty() && p.contains('='))
            .count()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.parameters.split(';').filter_map(|param| {
            let param = param.trim();
            if param.is_empty() {
                return None;
            }

            let eq_pos = param.find('=')?;
            let key = param[..eq_pos].trim();
            let value = Self::unquote(param[eq_pos + 1..].trim());
            Some((key, value))
        })
    }

    pub fn content_type(&self) -> Result<MimeType, Error> {
        MimeType::from_str(self.mime_type)
    }
}

impl fmt::Display for ParsedMimeType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[inline]
const fn is_valid_mime_token_byte(byte: u8) -> bool {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => true,
        b'/' | b'-' | b'.' | b'+' | b'_' => true,
        // b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => true,
        // Common MIME characters
        // b'/' | b'-' | b'.' | b'+' => true,
        // Whitespace (will be trimmed)
        // b' ' | b'\t' => true,
        // Everything else rejected (including Unicode, control chars, symbols)
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest::string::string_regex;

    fn valid_mime_token() -> impl Strategy<Value = String> {
        string_regex(r"[a-zA-Z0-9][a-zA-Z0-9\-\.\+_]*")
            .unwrap()
            .prop_filter("non-empty", |s| !s.is_empty() && s.len() <= 50)
    }

    fn known_mime_type() -> impl Strategy<Value = MimeType> {
        prop_oneof![
            any::<Application>().prop_map(MimeType::Application),
            any::<Audio>().prop_map(MimeType::Audio),
            any::<Chemical>().prop_map(MimeType::Chemical),
            any::<Font>().prop_map(MimeType::Font),
            any::<Image>().prop_map(MimeType::Image),
            any::<Message>().prop_map(MimeType::Message),
            any::<Model>().prop_map(MimeType::Model),
            any::<Multipart>().prop_map(MimeType::Multipart),
            any::<Text>().prop_map(MimeType::Text),
            any::<Video>().prop_map(MimeType::Video),
        ]
    }

    fn mime_parameters() -> impl Strategy<Value = String> {
        vec((valid_mime_token(), valid_mime_token()), 0..5).prop_map(|params| {
            if params.is_empty() {
                String::new()
            } else {
                params
                    .into_iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>()
                    .join("; ")
            }
        })
    }

    fn invalid_mime_type() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("".to_string()),
            string_regex(r"[a-zA-Z0-9]+").unwrap(),
            string_regex(r"[a-zA-Z0-9]+/[a-zA-Z0-9]+/[a-zA-Z0-9]+").unwrap(),
            string_regex(r"[a-zA-Z0-9]+/[\s\t\n]+").unwrap(),
            string_regex(r"[a-zA-Z0-9]{500}/[a-zA-Z0-9]{500}").unwrap(),
        ]
    }

    proptest! {
        #[test]
        fn known_mime_type_roundtrip(mime_type in known_mime_type()) {
            let string = mime_type.to_string();
            let lowercase = string.to_lowercase();
            let uppercase = string.to_uppercase();
            let mixed = string.clone().chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect::<String>();

            let parsed = MimeType::from_str(&string).unwrap();
            let parsed_low = MimeType::from_str(&lowercase).unwrap();
            let parsed_up = MimeType::from_str(&uppercase).unwrap();
            let parsed_mixed = MimeType::from_str(&mixed).unwrap();

            ParsedMimeType::parse_str(&string).unwrap();
            ParsedMimeType::parse_str(&lowercase).unwrap();
            ParsedMimeType::parse_str(&uppercase).unwrap();
            ParsedMimeType::parse_str(&mixed).unwrap();


            prop_assert_eq!(string.matches('/').count(), 1);
            prop_assert!(!string.is_empty());
            prop_assert!(!string.starts_with('/'));
            prop_assert!(!string.ends_with('/'));
            prop_assert!(string.is_ascii());
            prop_assert!(string.len() <= 200);

            prop_assert_eq!(mime_type.clone(), string.clone());
            prop_assert_eq!(mime_type.clone(), lowercase.clone());
            prop_assert_eq!(mime_type.clone(), uppercase.clone());
            prop_assert_eq!(mime_type.clone(), mixed.clone());

            prop_assert_eq!(string, mime_type.clone());
            prop_assert_eq!(lowercase, mime_type.clone());
            prop_assert_eq!(uppercase, mime_type.clone());
            prop_assert_eq!(mixed, mime_type.clone());

            prop_assert_eq!(mime_type.clone(), parsed.clone());
            prop_assert_eq!(mime_type.clone(), parsed_low.clone());
            prop_assert_eq!(mime_type.clone(), parsed_up.clone());
            prop_assert_eq!(mime_type.clone(), parsed_mixed.clone());

            prop_assert_eq!(parsed, mime_type.clone());
            prop_assert_eq!(parsed_low, mime_type.clone());
            prop_assert_eq!(parsed_up, mime_type.clone());
            prop_assert_eq!(parsed_mixed, mime_type.clone());

            let str = mime_type.as_str();
            let parsed = MimeType::from_str(str).unwrap();
            let parsed_low = MimeType::from_str(&str.to_lowercase()).unwrap();
            let parsed_up = MimeType::from_str(&str.to_uppercase()).unwrap();

            prop_assert_eq!(str.matches('/').count(), 1);
            prop_assert!(!str.is_empty());
            prop_assert!(!str.starts_with('/'));
            prop_assert!(!str.ends_with('/'));
            prop_assert!(str.is_ascii());
            prop_assert!(str.len() <= 200);

            prop_assert_eq!(mime_type.clone(), parsed.clone());
            prop_assert_eq!(mime_type.clone(), parsed_low.clone());
            prop_assert_eq!(mime_type.clone(), parsed_up.clone());

            prop_assert_eq!(parsed, mime_type.clone());
            prop_assert_eq!(parsed_low, mime_type.clone());
            prop_assert_eq!(parsed_up, mime_type.clone());

            let header_value = mime_type.clone().to_header_value();
            let parsed = MimeType::from_header_value(&header_value).unwrap();
            ParsedMimeType::parse(&header_value).unwrap();

            prop_assert!(header_value.to_str().is_ok());

            prop_assert_eq!(mime_type.clone(), header_value.clone());
            prop_assert_eq!(mime_type.clone(), parsed);

            prop_assert_eq!(header_value, mime_type);
        }

        #[test]
        fn mime_type_with_parameters_parsing(
            mime_type in known_mime_type(),
            params in mime_parameters()
        ) {
            let base_str = mime_type.to_string();
            let with_params = if params.is_empty() {
                base_str
            } else {
                format!("{base_str}; {params}")
            };

            let parsed = MimeType::from_str(&with_params);
            prop_assert!(parsed.is_ok());

            let parsed_mime = parsed.unwrap();
            if params.is_empty() {
                prop_assert_eq!(parsed_mime, mime_type);
            } else {
                prop_assert!(parsed_mime.as_str().starts_with(mime_type.as_str()));
            }
        }

        #[test]
        fn invalid_mime_types_rejected(invalid in invalid_mime_type()) {
            let result = MimeType::from_str(&invalid);
            prop_assert!(result.is_err());
        }


        #[test]
        fn parameter_extraction(
            mime_type in known_mime_type(),
            charset in valid_mime_token(),
            boundary in valid_mime_token()
        ) {
            let with_charset = format!("{mime_type}; charset={charset}");
            let with_boundary = format!("{mime_type}; boundary={boundary}");
            let with_both = format!("{mime_type}; charset={charset}; boundary={boundary}");

            prop_assert_eq!(MimeType::extract_charset(&with_charset), Some(charset.as_str()));
            prop_assert_eq!(MimeType::extract_boundary(&with_boundary), Some(boundary.as_str()));
            prop_assert_eq!(MimeType::extract_charset(&with_both), Some(charset.as_str()));
            prop_assert_eq!(MimeType::extract_boundary(&with_both), Some(boundary.as_str()));
        }

        #[test]
        fn whitespace(mime_type in known_mime_type()) {
            let base_str = mime_type.to_string();
            let with_leading_space = format!(" {base_str}");
            let with_trailing_space = format!("{base_str} ");
            let with_both_spaces = format!(" {base_str} ");

            prop_assert_eq!(MimeType::from_str(&with_leading_space).unwrap(), mime_type.clone());
            prop_assert_eq!(MimeType::from_str(&with_trailing_space).unwrap(), mime_type.clone());
            prop_assert_eq!(MimeType::from_str(&with_both_spaces).unwrap(), mime_type);
        }
    }

    #[test]
    fn parse_simple_mime_type() {
        let parsed = ParsedMimeType::parse_str("text/plain").unwrap();
        assert_eq!(parsed.mime_type, "text/plain");
        assert_eq!(parsed.parameters, "");
        assert_eq!(parsed.charset(), None);
        assert_eq!(parsed.boundary(), None);
    }

    #[test]
    fn parse_mime_type_with_charset() {
        let parsed = ParsedMimeType::parse_str("text/html; charset=utf-8").unwrap();
        assert_eq!(parsed.mime_type, "text/html");
        assert_eq!(parsed.parameters, "charset=utf-8");
        assert_eq!(parsed.charset(), Some("utf-8"));
    }

    #[test]
    fn parse_mime_type_with_boundary() {
        let parsed =
            ParsedMimeType::parse_str("multipart/form-data; boundary=----WebKitFormBoundary")
                .unwrap();
        assert_eq!(parsed.mime_type, "multipart/form-data");
        assert_eq!(parsed.boundary(), Some("----WebKitFormBoundary"));
    }

    #[test]
    fn parse_mime_type_with_multiple_parameters() {
        let parsed =
            ParsedMimeType::parse_str("text/html; charset=utf-8; boundary=test123").unwrap();
        assert_eq!(parsed.mime_type, "text/html");
        assert_eq!(parsed.charset(), Some("utf-8"));
        assert_eq!(parsed.boundary(), Some("test123"));
    }
}
