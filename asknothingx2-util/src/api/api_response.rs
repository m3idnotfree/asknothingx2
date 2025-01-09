use std::{fmt, marker::PhantomData};

use http::StatusCode;
use serde::{
    de::DeserializeOwned,
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

#[derive(Clone)]
pub struct APIResponse<T>
where
    T: DeserializeOwned,
{
    status_code: StatusCode,
    body: String,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Debug for APIResponse<T>
where
    T: DeserializeOwned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("APIResponse")
            .field("status_code", &self.status_code)
            .field("body", &self.body)
            .finish()
    }
}

impl<T> APIResponse<T>
where
    T: DeserializeOwned,
{
    pub fn new(status_code: StatusCode, body: String) -> Self {
        APIResponse {
            status_code,
            body,
            _phantom: PhantomData,
        }
    }

    pub async fn from_response(response: reqwest::Response) -> Result<Self, super::Error> {
        Ok(Self {
            status_code: response.status(),
            body: response.text().await?,
            _phantom: PhantomData,
        })
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn raw_body(&self) -> &str {
        &self.body
    }

    pub fn parse_body(self) -> Result<T, super::Error> {
        match self.status_code {
            StatusCode::OK => serde_json::from_str(&self.body)
                .map_err(|e| super::Error::DeserializationError(e.to_string())),
            _ => Err(super::Error::ResponseError(APIError::new(
                self.status(),
                self.body,
            ))),
        }
    }
}

#[derive(Debug)]
pub struct APIError {
    status_code: StatusCode,
    message: String,
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

    pub fn parse_body<T: DeserializeOwned>(self) -> Result<T, super::Error> {
        serde_json::from_str(&self.message)
            .map_err(|e| super::Error::DeserializationError(e.to_string()))
    }
}

#[derive(Debug)]
pub struct EmptyArrayResponse;

impl<'de> Deserialize<'de> for EmptyArrayResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
            type Value = EmptyArrayResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty array")
            }

            fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(EmptyArrayResponse)
            }
        }

        deserializer.deserialize_seq(EmptyVisitor)
    }
}
impl Serialize for EmptyArrayResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

#[derive(Debug)]
pub struct EmptyObjectResponse;

impl<'de> Deserialize<'de> for EmptyObjectResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
            type Value = EmptyObjectResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty object")
            }

            fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(EmptyObjectResponse)
            }
        }

        deserializer.deserialize_map(EmptyVisitor)
    }
}

impl Serialize for EmptyObjectResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = serializer.serialize_map(Some(0))?;
        map.end()
    }
}

#[derive(Debug)]
pub struct EmptyStringResponse;

impl<'de> Deserialize<'de> for EmptyStringResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmptyVisitor;

        impl serde::de::Visitor<'_> for EmptyVisitor {
            type Value = EmptyStringResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    Ok(EmptyStringResponse)
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

impl Serialize for EmptyStringResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("")
    }
}

#[cfg(test)]
mod tests {

    use crate::api::api_response::EmptyArrayResponse;

    use super::{EmptyObjectResponse, EmptyStringResponse};

    #[test]
    fn empty_response() {
        let empty1 = serde_json::from_str::<EmptyStringResponse>("\"\""); // empty string
        assert!(empty1.is_ok());
        let empty1 = empty1.unwrap();
        let empty1 = serde_json::to_string(&empty1).unwrap();
        //assert_eq!("f", empty1);
        assert!(empty1.contains("\"\""));

        let empty2 = serde_json::from_str::<EmptyObjectResponse>("{}"); // empty object
        assert!(empty2.is_ok());
        let empty2 = empty2.unwrap();
        let empty2 = serde_json::to_string(&empty2).unwrap();
        assert!(empty2.contains("{}"));

        let empty3 = serde_json::from_str::<EmptyArrayResponse>("[]"); // empty array
        assert!(empty3.is_ok());
        let empty3 = empty3.unwrap();
        let empty3 = serde_json::to_string(&empty3).unwrap();
        assert!(empty3.contains("[]"));

        let empty4 = serde_json::from_str::<EmptyStringResponse>(r#""""#); // quoted empty string
        assert!(empty4.is_ok());
        let empty4 = empty4.unwrap();
        let empty4 = serde_json::to_string(&empty4).unwrap();
        assert!(empty4.contains("\"\""));

        let fail1 = serde_json::from_str::<EmptyStringResponse>("content"); // non-empty string
        assert!(fail1.is_err());
        let fail2 = serde_json::from_str::<EmptyObjectResponse>("{\"k\",\"v\"}"); // non-empty object
        assert!(fail2.is_err());
        let fail3 = serde_json::from_str::<EmptyArrayResponse>("[1,2,3]"); // non-empty array
        assert!(fail3.is_err());
        let fail3 = serde_json::from_str::<EmptyObjectResponse>(r#"{"k":"v"}"#); // non-empty object
        assert!(fail3.is_err());
    }
}
