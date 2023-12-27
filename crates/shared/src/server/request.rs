use std::collections::HashMap;

use super::context::NgynContext;
use crate::enums::HttpMethod;

/// A struct that represents a request.
#[derive(Clone)]
pub struct NgynRequest {
    /// The HTTP method of the request.
    method: HttpMethod,
    /// The URL of the request.
    url: String,
    /// The headers of the request.
    headers: HashMap<String, String>,
    /// bytes format
    body: Option<Vec<u8>>,
    /// context for the request
    pub context: NgynContext,
}

impl NgynRequest {
    fn read_body(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if let Some(body) = &self.body {
            return Ok(body.clone());
        }
        self.body = None;
        panic!("Body has already been read");
    }

    /// Gets the body of the `NgynRequest`.
    pub fn body_string(&mut self) -> Result<String, std::io::Error> {
        Ok(String::from_utf8(self.read_body()?).unwrap())
    }

    /// Gets the HTTP method of the `NgynRequest`.
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Gets the URL of the `NgynRequest`.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Gets the headers of the `NgynRequest`.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

impl From<(String, String, HashMap<String, String>, Vec<u8>)> for NgynRequest {
    fn from(value: (String, String, HashMap<String, String>, Vec<u8>)) -> Self {
        let (method, url, headers, body) = value;
        Self {
            method: method.into(),
            url,
            headers,
            body: Some(body),
            context: NgynContext::default(),
        }
    }
}
