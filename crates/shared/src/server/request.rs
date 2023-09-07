use std::collections::HashMap;
use tide::Request;

use crate::enums::HttpMethod;

pub struct NgynRequest {
    request: Request<()>,
}

impl NgynRequest {
    /// Constructs a new `NgynRequest`.
    pub fn new(request: Request<()>) -> Self {
        Self { request }
    }

    /// Gets the method of the `NgynRequest`.
    pub fn method(&self) -> HttpMethod {
        let method = self.request.method().to_string();
        HttpMethod::from(method)
    }

    /// Gets the url of the `NgynRequest`.
    pub fn url(&self) -> &str {
        self.request.url().as_str()
    }

    /// Gets the headers of the `NgynRequest`.
    pub fn headers(&self) -> HashMap<String, String> {
        let mut headers_map = HashMap::new();
        for name in self.request.header_names() {
            if let Some(value) = self.request.header(name.as_str()) {
                headers_map.insert(name.to_string(), value.last().to_string());
            }
        }
        headers_map
    }

    /// Gets the body of the `NgynRequest`.
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
