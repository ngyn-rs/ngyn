use crate::{NgynRequest, NgynResponse};

pub type NextFn = fn(NgynRequest, NgynResponse);

/// Trait for implementing a middleware.
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    fn handle(&self, request: NgynRequest, response: NgynResponse, next: NextFn);
}
