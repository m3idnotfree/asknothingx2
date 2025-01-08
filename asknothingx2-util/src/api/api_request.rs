use http::{header::CONTENT_TYPE, HeaderMap, Method};
use url::Url;

use super::{reqwestError, reqwestResponse, ContentType};

pub trait APIRequest {
    fn method(&self) -> Method;
    fn url(&self) -> Url;
    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }
    fn json(&self) -> Option<String> {
        None
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        None
    }
    fn text(&self) -> Option<Vec<u8>> {
        None
    }
}

pub fn form_urlencoded_serialize(params: Vec<(&str, &str)>) -> Vec<u8> {
    url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes()
}

pub async fn api_request<T: APIRequest>(request: T) -> Result<reqwestResponse, reqwestError> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let headers = request.headers();
    let mut request_builder = client
        .request(request.method(), request.url())
        .headers(headers.clone());

    if let Some(body) = determine_body(&headers, &request) {
        request_builder = request_builder.body(match body {
            RequestBody::Json(json) => json.into_bytes(),
            RequestBody::FormEncoded(form) => form,
            RequestBody::Text(text) => text,
        });
    }

    request_builder.send().await
}

#[derive(Debug)]
enum RequestBody {
    Json(String),
    FormEncoded(Vec<u8>),
    Text(Vec<u8>),
}

fn determine_body<T: APIRequest>(headers: &HeaderMap, request: &T) -> Option<RequestBody> {
    let header_value = headers.get(CONTENT_TYPE)?;
    let content_type = ContentType::from_header_value(header_value)?;

    match content_type {
        ContentType::Json => request.json().map(RequestBody::Json),
        ContentType::FormEncoded => request.urlencoded().map(RequestBody::FormEncoded),
        ContentType::Text => request.text().map(RequestBody::Text),
    }
}
