use crate::{
    server::{NgynContext, NgynResponse},
    traits::NgynInjectable,
};

/// Trait for implementing a middleware.
pub trait NgynMiddleware: NgynInjectable + Sync {
    /// Handles the request.
    fn handle(&self, context: &mut NgynContext, response: &mut NgynResponse);
}
