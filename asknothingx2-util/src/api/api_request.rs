use super::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON, CONTENT_TYPE_TEXT};
use http::{header::CONTENT_TYPE, HeaderMap, Method};
use url::Url;

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

    fn form_urlencoded_serializere_pairs(params: Vec<(&str, &str)>) -> Vec<u8> {
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes()
    }
}

pub async fn api_request<T: APIRequest>(request: T) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let headers = request.headers();
    let client = client.request(request.method(), request.url());

    let client = if is_content_type_json(&headers) && request.json().is_some() {
        client.headers(headers).body(request.json().unwrap())
    } else if is_content_type_formenecoded(&headers) && request.urlencoded().is_some() {
        client.headers(headers).body(request.urlencoded().unwrap())
    } else if is_content_type_text(&headers) && request.text().is_some() {
        client.headers(headers).body(request.text().unwrap())
    } else {
        client.headers(headers)
    };

    let response = client.send().await?;

    Ok(response)
}

fn is_content_type_json(headers: &HeaderMap) -> bool {
    headers.get(CONTENT_TYPE).is_some() && headers.get(CONTENT_TYPE).unwrap() == CONTENT_TYPE_JSON()
}

fn is_content_type_formenecoded(headers: &HeaderMap) -> bool {
    headers.get(CONTENT_TYPE).is_some()
        && headers.get(CONTENT_TYPE).unwrap() == CONTENT_TYPE_FORMENCODED()
}

fn is_content_type_text(headers: &HeaderMap) -> bool {
    headers.get(CONTENT_TYPE).is_some() && headers.get(CONTENT_TYPE).unwrap() == CONTENT_TYPE_TEXT()
}

#[cfg(test)]
mod tests {

    use http::{header::CONTENT_TYPE, HeaderMap};

    use crate::api::{
        api_request::{is_content_type_formenecoded, is_content_type_json, is_content_type_text},
        CONTENT_TYPE_TEXT,
    };

    use super::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON};

    #[test]
    fn check_header() {
        let mut headers1 = HeaderMap::new();
        headers1.append(CONTENT_TYPE, CONTENT_TYPE_JSON());

        let mut headers2 = HeaderMap::new();
        headers2.append(CONTENT_TYPE, CONTENT_TYPE_FORMENCODED());

        let mut headers3 = HeaderMap::new();
        headers3.append(CONTENT_TYPE, CONTENT_TYPE_TEXT());

        assert!(is_content_type_json(&headers1));
        assert!(is_content_type_formenecoded(&headers2));
        assert!(is_content_type_text(&headers3));
    }
}
