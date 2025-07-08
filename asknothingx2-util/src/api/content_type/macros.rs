macro_rules! case_insensitive_match {
    ($input:expr, { $($pattern:expr => $result:expr),* $(,)? }) => {
        case_insensitive_match!(@in $input, {$($pattern => $result),* })
    };

    (@in $input:expr, { $pattern:expr => $result:expr $(, $rest_pattern:expr => $rest_result:expr)* $(,)?}) => {
        if $input.eq_ignore_ascii_case($pattern) {
            return Ok($result);
        }

        $(
            else if $input.eq_ignore_ascii_case($rest_pattern) {
                Ok($rest_result)
            }
        )*

        else {
            Err(crate::api::content_type::ContentTypeError::Unsupported($input.to_string()))
        }
    };
}

macro_rules! define_mime_type {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident {
            $(
                $variant:ident => {
                    const: $const_name:ident,
                    mime: $mime_type:literal,
                    extensions: [$($ext:literal),*]
                    $(, aliases: [$($alias:literal),* $(,)?])?
                    $(,)?
                }
            ),*
            $(,)?
        }
    ) => {
        use std::str::FromStr;

        use crate::api::content_type::{ContentType, ContentTypeError};
        use http::HeaderValue;

        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum_name {
            $($variant,)*
        }

        impl $enum_name {
            $(
                const $const_name: &'static str = $mime_type;
            )*

            #[inline]
            pub const fn as_static(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => Self::$const_name,
                    )*
                }
            }

            #[inline]
            pub const fn as_str(&self) -> &str {
                self.as_static()
            }

            #[inline]
            pub fn as_header_value(&self) -> HeaderValue {
                HeaderValue::from_static(self.as_static())
            }

            #[inline]
            pub fn to_header_value(self) -> HeaderValue {
                HeaderValue::from_static(self.as_static())
            }

            pub fn from_header_value(value: &HeaderValue) -> Result<Self, ContentTypeError> {
                let content_type = value.to_str().map_err(|_| ContentTypeError::InvalidUtf8)?;
                Self::from_str(content_type)
            }

            pub fn from_extension(ext: &str) -> Option<Self> {
                match ext.to_ascii_lowercase().as_str() {
                    $(
                        $(
                            $ext => Some(Self::$variant),
                        )*
                    )*
                    _ => None,
                }
            }

            #[inline]
            pub const fn extensions(&self) -> &[&str] {
                match self {
                    $(
                        Self::$variant => &[$($ext,)*],
                    )*
                }
            }

            #[inline]
            pub const fn primary_extension(&self) -> Option<&str> {
                self.extensions().first().copied()
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl TryFrom<&str> for $enum_name {
            type Error = ContentTypeError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = ContentTypeError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl TryFrom<&HeaderValue> for $enum_name {
            type Error = ContentTypeError;

            fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
                Self::from_header_value(value)
            }
        }


        impl From<$enum_name> for String {
            fn from(value: $enum_name) -> Self {
                value.to_string()
            }
        }

        impl From<$enum_name> for HeaderValue {
            fn from(value: $enum_name) -> Self {
                value.to_header_value()
            }
        }

        impl PartialEq<$enum_name> for String {
            fn eq(&self, other: &$enum_name) -> bool {
                self.as_str().eq_ignore_ascii_case(other.as_str())
            }
        }

        impl PartialEq<$enum_name> for &str {
            fn eq(&self, other: &$enum_name) -> bool {
                self.eq_ignore_ascii_case(other.as_str())
            }
        }

        impl PartialEq<ContentType> for $enum_name {
            fn eq(&self, other: &ContentType) -> bool {
                self.as_str().eq_ignore_ascii_case(other.as_str())
            }
        }

        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl FromStr for $enum_name {
            type Err = ContentTypeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.is_empty() {
                    return Err(ContentTypeError::Empty);
                }

                let mime_type = s.split(';').next().unwrap_or(s).trim();

                case_insensitive_match!(mime_type, {
                    $(
                        $mime_type => Self::$variant,
                    )*

                    $(
                        $(
                            $(
                                $alias => Self::$variant,
                            )*
                        )?
                    )*
                })
            }
        }


    };
}
