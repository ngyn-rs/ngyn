use super::Handler;
use crate::HttpMethod;

pub trait NgynEngine {
    /// Creates a new instance of `NgynApplication` with a new `Server`
    fn new() -> Self;

    /// Adds a new route to the `NgynApplication`.
    /// This function is chainable.
    ///
    /// ### Arguments
    ///
    /// * `path` - A string slice that represents the path of the route.
    /// * `method` - An `HttpMethod` that represents the HTTP method of the route.
    /// * `handler` - A closure that takes a `NgynRequest` and a `NgynResponse` and returns a `NgynResponse`.
    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self;
}
