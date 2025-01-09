use std::{fmt, marker::PhantomData};

use http::StatusCode;
use serde::de::DeserializeOwned;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenResponse")
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
    pub fn status(&self) -> StatusCode {
        self.status_code
    }
    pub fn raw_body(&self) -> &str {
        &self.body
    }
    pub fn parse_body<E: DeserializeOwned>(self) -> Result<T, super::Error> {
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
