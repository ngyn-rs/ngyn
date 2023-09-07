use tide::{Response, Result, StatusCode};

/// NgynResponse is a struct that represents a server response.
pub struct NgynResponse {
    response: Response,
}

impl NgynResponse {
    /// Constructs a new `NgynResponse` with a default status code of 200.
    pub fn new() -> Self {
        Self {
            response: Response::new(200),
        }
    }

    /// Sets the status code of the `NgynResponse`.
    ///
    /// # Arguments
    ///
    /// * `status` - A u16 that represents the status code to be set.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the `NgynResponse`.
    pub fn status(mut self, status: u16) -> Self {
        self.response
            .set_status(StatusCode::try_from(status).unwrap());
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
    /// * A mutable reference to the `NgynResponse`.
    pub fn body(mut self, data: &str) -> Self {
        self.response.set_body(data);
        self
    }

    /// Builds the `NgynResponse`.
    pub fn build(self) -> Result {
        Ok(self.response)
    }
}

impl From<Response> for NgynResponse {
    fn from(response: Response) -> Self {
        Self { response }
    }
}

impl Default for NgynResponse {
    fn default() -> Self {
        Self::new()
    }
}
