use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Clone)]
pub struct AppType {
    inner: AppTypeInner,
}

#[derive(Debug, Clone)]
enum AppTypeInner {
    Static(&'static str),
    Dynamic(Box<str>),
}

impl AppType {
    pub const fn from_static(name: &'static str) -> Self {
        Self {
            inner: AppTypeInner::Static(name),
        }
    }

    pub fn try_from_str(name: &str) -> Result<Self, InvalidAppType> {
        Ok(Self {
            inner: AppTypeInner::Dynamic(name.to_string().into_boxed_str()),
        })
    }

    pub fn try_from_string(name: String) -> Result<Self, InvalidAppType> {
        validate_app_type(&name)?;
        Ok(Self {
            inner: AppTypeInner::Dynamic(name.into_boxed_str()),
        })
    }

    pub fn as_str(&self) -> &str {
        match &self.inner {
            AppTypeInner::Static(s) => s,
            AppTypeInner::Dynamic(s) => s,
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(self.inner, AppTypeInner::Static(_))
    }
}

impl AppType {
    pub const WEB: AppType = AppType::from_static("web");
    pub const CLI: AppType = AppType::from_static("cli");
    pub const PRODUCTION: AppType = AppType::from_static("production");
    pub const DEVELOPMENT: AppType = AppType::from_static("development");
    pub const GATEWAY: AppType = AppType::from_static("gateway");
    pub const SCRAPING: AppType = AppType::from_static("scraping");
}

impl fmt::Debug for AppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for AppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for AppType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for AppType {
    fn eq(&self, other: &AppType) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
}

impl Eq for AppType {}

impl Hash for AppType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().to_lowercase().hash(state);
    }
}

impl PartialEq<str> for AppType {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<&str> for AppType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<String> for AppType {
    fn eq(&self, other: &String) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

impl TryFrom<&str> for AppType {
    type Error = InvalidAppType;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        AppType::try_from_str(name)
    }
}

impl TryFrom<String> for AppType {
    type Error = InvalidAppType;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        AppType::try_from_string(name)
    }
}

impl FromStr for AppType {
    type Err = InvalidAppType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AppType::try_from_str(s)
    }
}

const RESERVED_NAMES: &[&str] = &[
    "web",
    "cli",
    "production",
    "development",
    "gateway",
    "scraping",
];

fn validate_app_type(name: &str) -> Result<(), InvalidAppType> {
    if name.is_empty() {
        return Err(InvalidAppType::empty());
    }

    if name.len() > 64 {
        return Err(InvalidAppType::too_long(name.len()));
    }

    for (pos, ch) in name.char_indices() {
        if !is_valid_char(ch) {
            return Err(InvalidAppType::invalid_character(ch, pos));
        }
    }

    let name_lower = name.to_ascii_lowercase();
    if RESERVED_NAMES.contains(&name_lower.as_str()) {
        return Err(InvalidAppType::reserved(name.to_string()));
    }

    Ok(())
}

fn is_valid_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

#[derive(Debug, Clone)]
pub struct AppTypeMap<V> {
    inner: HashMap<AppType, V>,
}

impl<V> AppTypeMap<V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: AppType, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &AppType) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn get_by_str(&self, key: &str) -> Option<&V> {
        self.inner
            .iter()
            .find(|(k, _)| k.as_str().eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &AppType) -> bool {
        self.inner.contains_key(key)
    }

    pub fn contains_key_str(&self, key: &str) -> bool {
        self.inner
            .keys()
            .any(|k| k.as_str().eq_ignore_ascii_case(key))
    }

    pub fn remove(&mut self, key: &AppType) -> Option<V> {
        self.inner.remove(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &AppType> {
        self.inner.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl<V> Default for AppTypeMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidAppType {
    kind: InvalidAppTypeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InvalidAppTypeKind {
    Empty,
    TooLong(usize),
    InvalidCharacter(char, usize),
    Reserved(String),
}

impl InvalidAppType {
    pub fn empty() -> Self {
        Self {
            kind: InvalidAppTypeKind::Empty,
        }
    }
    pub fn too_long(len: usize) -> Self {
        Self {
            kind: InvalidAppTypeKind::TooLong(len),
        }
    }

    pub fn invalid_character(ch: char, pos: usize) -> Self {
        Self {
            kind: InvalidAppTypeKind::InvalidCharacter(ch, pos),
        }
    }

    pub fn reserved(name: impl Into<String>) -> Self {
        Self {
            kind: InvalidAppTypeKind::Reserved(name.into()),
        }
    }
}

impl fmt::Display for InvalidAppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            InvalidAppTypeKind::Empty => f.write_str("app type cannot be empty"),
            InvalidAppTypeKind::TooLong(len) => {
                write!(f, "app type too long: {len} characters (max 64)")
            }
            InvalidAppTypeKind::InvalidCharacter(ch, pos) => {
                write!(f, "invalid character '{ch}' at position {pos}")
            }
            InvalidAppTypeKind::Reserved(name) => {
                write!(f, "app type '{name}' is reserved")
            }
        }
    }
}

impl std::error::Error for InvalidAppType {}
