use crate::{NgynRequest, NgynResponse};

pub type NextFn = Option<std::sync::Arc<dyn NgynMiddleware>>;

/// Trait for implementing a middleware.
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    fn handle(&self, request: &NgynRequest, response: &NgynResponse);
}
