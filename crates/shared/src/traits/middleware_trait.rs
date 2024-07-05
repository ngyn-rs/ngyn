use crate::{
    server::{NgynContext, NgynResponse},
    traits::NgynInjectable,
};

/// Trait for implementing a middleware.
pub trait NgynMiddleware<'a>: NgynInjectable + Sync {
    /// Handles the request.
    fn handle(&self, context: &'a mut NgynContext, response: &'a mut NgynResponse);
}
