use http::HeaderValue;

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Json,
    FormEncoded,
    Text,
}

impl ContentType {
    const JSON_STR: &'static str = "application/json";
    const FORM_STR: &'static str = "application/x-www-form-urlencoded";
    const TEXT_STR: &'static str = "text/plain";

    pub fn as_header_value(&self) -> HeaderValue {
        match self {
            ContentType::Json => HeaderValue::from_static(Self::JSON_STR),
            ContentType::FormEncoded => HeaderValue::from_static(Self::FORM_STR),
            ContentType::Text => HeaderValue::from_static(Self::TEXT_STR),
        }
    }

    pub fn from_header_value(value: &HeaderValue) -> Option<Self> {
        match value.to_str().ok()? {
            Self::JSON_STR => Some(ContentType::Json),
            Self::FORM_STR => Some(ContentType::FormEncoded),
            Self::TEXT_STR => Some(ContentType::Text),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::ContentType;

    #[test]
    fn content_type_json() {
        assert_eq!(
            ContentType::Json.as_header_value().to_str().unwrap(),
            "application/json"
        );
    }

    #[test]
    fn content_type_formencoded() {
        assert_eq!(
            ContentType::FormEncoded.as_header_value().to_str().unwrap(),
            "application/x-www-form-urlencoded"
        );
    }

    #[test]
    fn content_type_text() {
        assert_eq!(
            ContentType::Text.as_header_value().to_str().unwrap(),
            "text/plain",
        );
    }
}
