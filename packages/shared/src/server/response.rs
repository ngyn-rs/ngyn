/// RustleResponse is a struct that represents a server response.
pub struct RustleResponse {
    response: tide::Response,
}

impl RustleResponse {
    /// Constructs a new `RustleResponse` with a default status code of 200.
    pub fn new() -> Self {
        Self {
            response: tide::Response::new(200),
        }
    }

    /// Sets the status code of the `RustleResponse`.
    ///
    /// # Arguments
    ///
    /// * `status` - A u16 that represents the status code to be set.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `RustleResponse`.
    pub fn status(mut self, status: u16) -> Self {
        self.response
            .set_status(tide::StatusCode::try_from(status).unwrap());
        self
    }

    /// Sets the body of the response
    ///
    /// # Arguments
    ///
    /// * `data` - A string that represents the body
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `RustleResponse`.
    pub fn body(mut self, data: &str) -> Self {
        self.response.set_body(data);
        self
    }

    /// Builds the `RustleResponse`.
    pub fn build(self) -> tide::Result {
        Ok(self.response)
    }
}
