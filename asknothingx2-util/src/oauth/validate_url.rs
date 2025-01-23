use std::ops::Deref;
use url::Url;

/// https://docs.rs/oauth2/latest/src/oauth2/types.rs.html#233
#[derive(Clone)]
pub struct ValidateUrl(Url, String);

impl ValidateUrl {
    pub fn new(url: String) -> std::result::Result<Self, url::ParseError> {
        Ok(Self(Url::parse(&url)?, url))
    }
    pub fn from_url(url: Url) -> Self {
        let s = url.to_string();
        Self(url, s)
    }
    pub fn url(&self) -> &Url {
        &self.0
    }
}

impl Deref for ValidateUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.1
    }
}

impl std::fmt::Debug for ValidateUrl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut debug_trait_builder = f.debug_tuple(stringify!(ValidateURL));
        debug_trait_builder.field(&self.1);
        debug_trait_builder.finish()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateUrl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        struct UrlVisitor;
        impl serde::de::Visitor<'_> for UrlVisitor {
            type Value = ValidateUrl;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(stringify!(ValidateUrl))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                ValidateUrl::new(v.to_string()).map_err(E::custom)
            }
        }
        deserializer.deserialize_str(UrlVisitor {})
    }
}

impl serde::Serialize for ValidateUrl {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: ::serde::Serializer,
    {
        serializer.serialize_str(&self.1)
    }
}

impl std::hash::Hash for ValidateUrl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&(self.1), state);
    }
}

impl Ord for ValidateUrl {
    fn cmp(&self, other: &ValidateUrl) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for ValidateUrl {
    fn partial_cmp(&self, other: &ValidateUrl) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ValidateUrl {
    fn eq(&self, other: &ValidateUrl) -> bool {
        self.1 == other.1
    }
}

impl Eq for ValidateUrl {}
