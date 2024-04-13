use crate::{NgynContext, NgynResponse};

/// Trait for implementing a middleware.
pub trait NgynMiddleware: Send + Sync {
    /// Handles the request.
    fn handle(&self, request: &mut NgynContext, response: &mut NgynResponse);
}
