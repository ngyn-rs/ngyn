use crate::{NgynRequest, NgynResponse};

/// Trait for implementing a middleware.
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    fn handle(&self, request: &NgynRequest, response: &NgynResponse);
}
