pub struct RustleRequest {
    request: tide::Request,
}

impl RustleRequest {
    /// Constructs a new `RustleRequest`.
    pub fn new() -> Self {
        Self {
            request: tide::Request::new(),
        }
    }

    /// Gets the method of the `RustleRequest`.
    pub fn method(&self) -> &tide::http::Method {
        self.request.method()
    }

    /// Gets the url of the `RustleRequest`.
    pub fn url(&self) -> &tide::http::Url {
        self.request.url()
    }

    /// Gets the headers of the `RustleRequest`.
    pub fn headers(&self) -> &tide::http::Headers {
        self.request.headers()
    }

    /// Gets the body of the `RustleRequest`.
    pub async fn body_string(&mut self) -> tide::Result<String> {
        self.request.body_string().await
    }
}
