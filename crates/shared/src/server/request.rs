use std::collections::HashMap;
use tide::Request;

use crate::enums::HttpMethod;

pub struct RustleRequest {
    request: Request<()>,
}

impl RustleRequest {
    /// Constructs a new `RustleRequest`.
    pub fn new(request: Request<()>) -> Self {
        Self { request }
    }

    /// Gets the method of the `RustleRequest`.
    pub fn method(&self) -> HttpMethod {
        let method = self.request.method().to_string();
        HttpMethod::from_str(method.as_str()).unwrap()
    }

    /// Gets the url of the `RustleRequest`.
    pub fn url(&self) -> &str {
        self.request.url().as_str()
    }

    /// Gets the headers of the `RustleRequest`.
    pub fn headers(&self) -> HashMap<String, String> {
        let mut headers_map = HashMap::new();
        for name in self.request.header_names() {
            if let Some(value) = self.request.header(name.as_str()) {
                headers_map.insert(name.to_string(), value.last().to_string());
            }
        }
        headers_map
    }

    /// Gets the body of the `RustleRequest`.
    pub async fn body_string(&mut self) -> Result<String, std::io::Error> {
        match self.request.body_string().await {
            Ok(body) => Ok(body),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to read body",
            )),
        }
    }
}
