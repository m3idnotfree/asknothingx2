use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use url::Url;
use zeroize::{Zeroize, ZeroizeOnDrop};

macro_rules! regular_type {
    (
    $(#[$attr:meta])*
    $name:ident
    ) => (
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $(#[$attr])*
        pub struct  $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str(self.as_str())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }


    )
}

macro_rules! secret_type {
    (
    $(#[$attr:meta])*
    $name:ident
    ) => (
        #[derive(Clone, Serialize, Deserialize)]
        #[derive(Zeroize, ZeroizeOnDrop)]
        #[serde(transparent)]
        $(#[$attr])*
        pub struct  $name(String);

        impl $name {
            pub fn secret(&self) -> &str {
                &self.0
            }

            pub fn into_secret(mut self) -> String {
                std::mem::take(&mut self.0)
            }

            pub fn clear(&mut self) {
                self.zeroize();
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("[REDACTED]")

            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.as_bytes().ct_eq(other.0.as_bytes()).into()
            }
        }

        impl Eq for $name {}

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                Sha256::digest(self.secret()).hash(state);
            }
        }

    )
}

macro_rules! url_type {
    (
        $(#[$attr:meta])*
        $name:ident
    ) => (
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        $(#[$attr])*
        pub struct  $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn to_url(&self) -> Url {
                Url::parse(&self.0).unwrap()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = url::ParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                url::Url::parse(s)?;
                Ok(Self(s.to_string()))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }
    )
}

regular_type!(ClientId);
regular_type!(Scope);
regular_type!(CsrfToken);

secret_type!(ClientSecret);
secret_type!(AuthorizationCode);
secret_type!(RefreshToken);
secret_type!(AccessToken);

url_type!(AuthUrl);
url_type!(TokenUrl);
url_type!(RedirectUrl);
url_type!(RevocationUrl);
url_type!(ValidateUrl);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::oauth::{AccessToken, AuthUrl, ClientId, ClientSecret, RedirectUrl};

    #[test]
    fn regular_type() {
        let client_id = ClientId::from("client_id");

        let json = serde_json::to_string(&client_id).unwrap();
        assert_eq!(json, "\"client_id\"");

        let deserialized: ClientId = serde_json::from_str(&json).unwrap();
        assert_eq!(client_id, deserialized);

        let client_id2 = ClientId::from("client_id");
        let client_id3 = ClientId::from("client_id3");
        assert_eq!(client_id, client_id2);
        assert_ne!(client_id, client_id3);

        let as_str = client_id.as_str();
        assert_eq!(as_str, "client_id");

        let display = client_id.to_string();
        assert_eq!(display, "client_id");

        assert_eq!(&*client_id, "client_id");
        assert_eq!(client_id.len(), 9);
    }

    #[test]
    fn secret_type() {
        let client_secret = ClientSecret::from("client_secret");

        let json = serde_json::to_string(&client_secret).unwrap();
        assert_eq!(json, "\"client_secret\"");

        let deserialized: ClientSecret = serde_json::from_str(&json).unwrap();
        assert_eq!(client_secret, deserialized);

        let client_secret2 = ClientSecret::from("client_secret");
        let client_secret3 = ClientSecret::from("client_secret3");
        assert_eq!(client_secret, client_secret2);
        assert_ne!(client_secret, client_secret3);

        let secret = client_secret.secret();
        assert_eq!(secret, "client_secret");

        let display = client_secret.to_string();
        assert_eq!(display, "[REDACTED]");

        assert_eq!(&*client_secret, "client_secret");
        assert_eq!(client_secret.len(), 13);

        let mut token = AccessToken::from("access_token");

        token.clear();

        assert_eq!(token.secret(), "");
    }

    #[test]
    fn url_type() {
        let auth_url = AuthUrl::from_str("https://id.twitch.tv/oauth2/authorize").unwrap();

        let json = serde_json::to_string(&auth_url).unwrap();
        assert_eq!(json, "\"https://id.twitch.tv/oauth2/authorize\"");

        let deserialized: AuthUrl = serde_json::from_str(&json).unwrap();
        assert_eq!(auth_url, deserialized);

        let auth_url2 = AuthUrl::from_str("https://id.twitch.tv/oauth2/authorize").unwrap();
        assert_eq!(auth_url, auth_url2);

        let as_str = auth_url.as_str();
        assert_eq!(as_str, "https://id.twitch.tv/oauth2/authorize");

        let display = auth_url.to_string();
        assert_eq!(display, "https://id.twitch.tv/oauth2/authorize");

        assert_eq!(&*auth_url, "https://id.twitch.tv/oauth2/authorize");

        let valid = RedirectUrl::from_str("not_a_ual");
        assert!(valid.is_err());
    }
}
