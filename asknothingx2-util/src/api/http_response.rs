use http::{HeaderMap, StatusCode};

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}
