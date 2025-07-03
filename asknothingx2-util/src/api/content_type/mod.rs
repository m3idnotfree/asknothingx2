#[macro_use]
mod macros;

mod application;
mod audio;
mod chemical;
mod error;
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
pub use error::ContentTypeError;
pub use font::Font;
pub use image::Image;
pub use message::Message;
pub use model::Model;
pub use multipart::Multipart;
pub use text::Text;
pub use video::Video;

use std::{fmt, str::FromStr};

use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentType {
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

impl ContentType {
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

    pub fn from_header_value(value: &HeaderValue) -> Result<Self, ContentTypeError> {
        let content_type = value.to_str().map_err(|_| ContentTypeError::InvalidUtf8)?;
        Self::from_str(content_type)
    }

    pub fn extract_charset(content_type: &str) -> Option<&str> {
        ParsedContentType::parse_str(content_type).ok()?.charset()
    }

    pub fn extract_boundary(content_type: &str) -> Option<&str> {
        ParsedContentType::parse_str(content_type).ok()?.boundary()
    }

    pub fn matches_with_params(content_type: &str, expected: Self) -> bool {
        Self::from_str(content_type)
            .map(|parsed| parsed == expected)
            .unwrap_or(false)
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        if let Some(text) = Text::from_extension(ext) {
            return Some(Self::Text(text));
        }

        if let Some(app) = Application::from_extension(ext) {
            return Some(Self::Application(app));
        }

        if let Some(image) = Image::from_extension(ext) {
            return Some(Self::Image(image));
        }

        if let Some(video) = Video::from_extension(ext) {
            return Some(Self::Video(video));
        }

        if let Some(audio) = Audio::from_extension(ext) {
            return Some(Self::Audio(audio));
        }

        if let Some(font) = Font::from_extension(ext) {
            return Some(Self::Font(font));
        }

        if let Some(model) = Model::from_extension(ext) {
            return Some(Self::Model(model));
        }

        if let Some(chemical) = Chemical::from_extension(ext) {
            return Some(Self::Chemical(chemical));
        }

        if let Some(message) = Message::from_extension(ext) {
            return Some(Self::Message(message));
        }

        if let Some(multipart) = Multipart::from_extension(ext) {
            return Some(Self::Multipart(multipart));
        }

        None
    }

    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    pub fn from_filename(filename: &str) -> Option<Self> {
        filename
            .rfind('.')
            .and_then(|pos| filename.get(pos + 1..))
            .and_then(Self::from_extension)
    }

    pub fn is_extension_supported(ext: &str) -> bool {
        Self::from_extension(ext).is_some()
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

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ContentType {
    type Err = ContentTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = ParsedContentType::parse_str(s)?;

        let mime_type = parsed.mime_type;

        if let Ok(s) = Text::from_str(s) {
            return Ok(ContentType::Text(s));
        }

        if let Ok(s) = Application::from_str(mime_type) {
            return Ok(ContentType::Application(s));
        }

        if let Ok(s) = Image::from_str(s) {
            return Ok(ContentType::Image(s));
        }

        if let Ok(s) = Video::from_str(s) {
            return Ok(ContentType::Video(s));
        }

        if let Ok(s) = Audio::from_str(s) {
            return Ok(ContentType::Audio(s));
        }

        if let Ok(s) = Font::from_str(s) {
            return Ok(ContentType::Font(s));
        }

        if let Ok(s) = Model::from_str(s) {
            return Ok(ContentType::Model(s));
        }

        if let Ok(s) = Chemical::from_str(s) {
            return Ok(ContentType::Chemical(s));
        }

        if let Ok(s) = Message::from_str(s) {
            return Ok(ContentType::Message(s));
        }

        if let Ok(s) = Multipart::from_str(s) {
            return Ok(ContentType::Multipart(s));
        }

        Ok(ContentType::Custom(s.to_string()))
    }
}

impl TryFrom<&str> for ContentType {
    type Error = ContentTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for ContentType {
    type Error = ContentTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&HeaderValue> for ContentType {
    type Error = ContentTypeError;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        Self::from_header_value(value)
    }
}

impl From<ContentType> for String {
    fn from(value: ContentType) -> Self {
        value.to_string()
    }
}

impl From<ContentType> for HeaderValue {
    fn from(value: ContentType) -> Self {
        value.to_header_value()
    }
}

impl<T> PartialEq<T> for ContentType
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

#[derive(Debug, Clone)]
struct ParsedContentType<'a> {
    pub mime_type: &'a str,
    parameters: &'a str,
}

impl<'a> ParsedContentType<'a> {
    pub fn parse(header_value: &'a HeaderValue) -> Result<Self, ContentTypeError> {
        let content_type_str = header_value
            .to_str()
            .map_err(|_| ContentTypeError::InvalidUtf8)?;
        Self::parse_str(content_type_str)
    }

    pub fn parse_str(input: &'a str) -> Result<Self, ContentTypeError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ContentTypeError::Empty);
        }

        if let Some(semicolon_pos) = input.find(';') {
            let mime_type = input[..semicolon_pos].trim();
            let parameters = input[semicolon_pos + 1..].trim();

            if !Self::is_valid_mime_type(mime_type) {
                return Err(ContentTypeError::InvalidMimeType(mime_type.to_string()));
            }

            Ok(Self {
                mime_type,
                parameters,
            })
        } else {
            if !Self::is_valid_mime_type(input) {
                return Err(ContentTypeError::InvalidMimeType(input.to_string()));
            }

            Ok(Self {
                mime_type: input,
                parameters: "",
            })
        }
    }

    fn is_valid_mime_type(mime_type: &str) -> bool {
        if mime_type.is_empty() || mime_type.len() > 200 {
            return false;
        }

        let bytes = mime_type.as_bytes();
        let mut slash_pos = None;

        for (i, &byte) in bytes.iter().enumerate() {
            if !is_valid_mime_byte(byte) {
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

    pub fn content_type(&self) -> Result<ContentType, ContentTypeError> {
        ContentType::from_str(self.mime_type)
    }
}

#[inline]
const fn is_valid_mime_byte(byte: u8) -> bool {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => true,
        // Common MIME characters
        b'/' | b'-' | b'.' | b'+' => true,
        // Whitespace (will be trimmed)
        b' ' | b'\t' => true,
        // Everything else rejected (including Unicode, control chars, symbols)
        _ => false,
    }
}
