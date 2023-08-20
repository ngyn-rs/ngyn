use rustle_shared::enums::http_method_enum::HttpMethod

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
    pub fn method(&self) -> &HttpMethod {
        &HttpMethod::from_str(self.request.method().to_string())
    }

    /// Gets the url of the `RustleRequest`.
    pub fn url(&self) -> &str {
        self.request.url().as_str()
    }

    /// Gets the headers of the `RustleRequest`.
    pub fn headers(&self) -> HashMap<String, String> {
        self.request
            .headers()
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect()
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
