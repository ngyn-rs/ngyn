use std::{
    collections::HashMap,
    future::Future,
    task::{Context, Poll},
};
use tide::Request;

use crate::enums::HttpMethod;

#[derive(Clone)]
pub enum Body {
    Raw(Vec<u8>),
}

impl From<Vec<u8>> for Body {
    fn from(raw: Vec<u8>) -> Self {
        Self::Raw(raw)
    }
}

impl Future for Body {
    type Output = Result<String, std::io::Error>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Result<String, std::io::Error>> {
        match self.get_mut() {
            Body::Raw(raw) => Poll::Ready(Ok(String::from_utf8(raw.clone()).unwrap())),
        }
    }
}

/// A struct that represents a request.
#[derive(Clone)]
pub struct NgynRequest {
    /// The HTTP method of the request.
    method: HttpMethod,
    /// The URL of the request.
    url: String,
    /// The headers of the request.
    headers: HashMap<String, String>,
    body: Body,
}

impl NgynRequest {
    /// Constructs a new `NgynRequest`.
    pub fn new(request: Request<()>) -> Self {
        Self {
            method: HttpMethod::from(request.method().to_string()),
            url: request.url().to_string(),
            headers: {
                let mut headers_map = HashMap::new();
                for name in request.header_names() {
                    if let Some(value) = request.header(name.as_str()) {
                        headers_map.insert(name.to_string(), value.last().to_string());
                    }
                }
                headers_map
            },
            body: Body::Raw(Vec::new()),
        }
    }

    /// Gets the body of the `NgynRequest`.
    pub async fn body_string(&mut self) -> Result<String, std::io::Error> {
        self.body.clone().await
    }

    /// Gets the HTTP method of the `NgynRequest`.
    pub fn method(&self) -> HttpMethod {
        self.method.clone()
    }

    /// Gets the URL of the `NgynRequest`.
    pub fn url(&self) -> String {
        self.url.clone()
    }

    /// Gets the headers of the `NgynRequest`.
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

impl From<Request<()>> for NgynRequest {
    fn from(request: Request<()>) -> Self {
        Self::new(request)
    }
}

impl From<(String, String, HashMap<String, String>, Vec<u8>)> for NgynRequest {
    fn from(value: (String, String, HashMap<String, String>, Vec<u8>)) -> Self {
        let (method, url, headers, body) = value;
        Self {
            method: HttpMethod::from(method),
            url,
            headers,
            body: Body::Raw(body),
        }
    }
}
