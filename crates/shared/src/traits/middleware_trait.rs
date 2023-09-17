use crate::{NgynRequest, NgynResponse};

pub type NextFn = fn(NgynRequest, NgynResponse);

/// Trait for implementing a middleware.
pub trait NgynMiddleware {
    /// Handles the request.
    fn handle(request: NgynRequest, response: NgynResponse, next: NextFn);
}
