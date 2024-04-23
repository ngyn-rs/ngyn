use super::Handler;
use crate::Method;

pub trait NgynEngine: Default {

    /// Adds a new route to the `App`.
    /// This function is chainable.
    ///
    /// ### Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `Method` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `NgynContext` and a `NgynResponse` and returns a `NgynResponse`.
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>) -> &mut Self;
}
