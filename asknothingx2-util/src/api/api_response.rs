use std::fmt;

use bytes::Bytes;
use http::StatusCode;
use serde::{
    de::DeserializeOwned,
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

use super::error::JsonError;

#[derive(Clone)]
pub struct APIResponse {
    status_code: StatusCode,
    body: Bytes,
}

impl fmt::Debug for APIResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("APIResponse")
            .field("status_code", &self.status_code)
            .field("body", &self.body)
            .finish()
    }
}

impl APIResponse {
    pub fn new(status_code: StatusCode, body: Bytes) -> Self {
        APIResponse { status_code, body }
    }

    pub async fn from_response(response: reqwest::Response) -> Result<Self, super::ReqwestError> {
        Ok(Self {
            status_code: response.status(),
            body: response.bytes().await?,
        })
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn raw_body(&self) -> &Bytes {
        &self.body
    }
    pub fn is_success(&self) -> bool {
        self.status_code.is_success()
    }

    pub fn into_json<T: DeserializeOwned>(self) -> Result<T, JsonError> {
        Ok(serde_json::from_slice(&self.body)?)
    }
}

#[derive(Debug)]
pub struct APIError {
    status_code: StatusCode,
    message: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "API Error ({}): {}",
            self.status_code.as_u16(),
            self.message
        )
    }
}

impl APIError {
    pub fn new(status_code: StatusCode, body: String) -> Self {
        Self {
            status_code,
            message: body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn raw_message(&self) -> &str {
        &self.message
    }

    pub fn parse_body<T: DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.message)
    }
}

#[derive(Debug)]
pub struct EmptyArrayBody;

impl<'de> Deserialize<'de> for EmptyArrayBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
            type Value = EmptyArrayBody;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty array")
            }

            fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(EmptyArrayBody)
            }
        }

        deserializer.deserialize_seq(EmptyVisitor)
    }
}
impl Serialize for EmptyArrayBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

#[derive(Debug)]
pub struct EmptyObjectBody;

impl<'de> Deserialize<'de> for EmptyObjectBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
            type Value = EmptyObjectBody;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty object")
            }

            fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(EmptyObjectBody)
            }
        }

        deserializer.deserialize_map(EmptyVisitor)
    }
}

impl Serialize for EmptyObjectBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = serializer.serialize_map(Some(0))?;
        map.end()
    }
}

#[derive(Debug)]
pub struct EmptyStringBody;

impl<'de> Deserialize<'de> for EmptyStringBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl serde::de::Visitor<'_> for EmptyVisitor {
            type Value = EmptyStringBody;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    Ok(EmptyStringBody)
                } else {
                    Err(E::custom("expected empty string"))
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(EmptyVisitor)
    }
}

impl Serialize for EmptyStringBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("")
    }
}

#[cfg(test)]
mod tests {

    use crate::api::api_response::EmptyArrayBody;

    use super::{EmptyObjectBody, EmptyStringBody};

    #[test]
    fn empty_response() {
        let empty1 = serde_json::from_str::<EmptyStringBody>("\"\""); // empty string
        assert!(empty1.is_ok());
        let empty1 = empty1.unwrap();
        let empty1 = serde_json::to_string(&empty1).unwrap();
        //assert_eq!("f", empty1);
        assert!(empty1.contains("\"\""));

        let empty2 = serde_json::from_str::<EmptyObjectBody>("{}"); // empty object
        assert!(empty2.is_ok());
        let empty2 = empty2.unwrap();
        let empty2 = serde_json::to_string(&empty2).unwrap();
        assert!(empty2.contains("{}"));

        let empty3 = serde_json::from_str::<EmptyArrayBody>("[]"); // empty array
        assert!(empty3.is_ok());
        let empty3 = empty3.unwrap();
        let empty3 = serde_json::to_string(&empty3).unwrap();
        assert!(empty3.contains("[]"));

        let empty4 = serde_json::from_str::<EmptyStringBody>(r#""""#); // quoted empty string
        assert!(empty4.is_ok());
        let empty4 = empty4.unwrap();
        let empty4 = serde_json::to_string(&empty4).unwrap();
        assert!(empty4.contains("\"\""));

        let fail1 = serde_json::from_str::<EmptyStringBody>("content"); // non-empty string
        assert!(fail1.is_err());
        let fail2 = serde_json::from_str::<EmptyObjectBody>("{\"k\",\"v\"}"); // non-empty object
        assert!(fail2.is_err());
        let fail3 = serde_json::from_str::<EmptyArrayBody>("[1,2,3]"); // non-empty array
        assert!(fail3.is_err());
        let fail3 = serde_json::from_str::<EmptyObjectBody>(r#"{"k":"v"}"#); // non-empty object
        assert!(fail3.is_err());
    }
}
