#![cfg(all(test, feature = "api", feature = "stream"))]
use std::sync::{Arc, Mutex};

use wiremock::{Match, Request};

#[derive(Debug, Clone)]
struct BodyCapture {
    captured_body: Arc<Mutex<Option<Vec<u8>>>>,
}

impl BodyCapture {
    fn new() -> Self {
        Self {
            captured_body: Arc::new(Mutex::new(None)),
        }
    }

    fn get_captured_body(&self) -> Option<Vec<u8>> {
        self.captured_body.lock().unwrap().clone()
    }
}

impl Match for BodyCapture {
    fn matches(&self, request: &Request) -> bool {
        let body = request.body.clone();
        *self.captured_body.lock().unwrap() = Some(body);
        true
    }
}
