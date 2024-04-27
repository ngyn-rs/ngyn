use super::Handler;
use crate::{Method, NgynMiddleware};

pub trait NgynEngine: Default {
    /// Adds a route to the application.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `method` - The HTTP method of the route.
    /// * `handler` - The handler function for the route.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use crate::{Method, NgynEngine};
    ///
    /// struct MyEngine;
    ///
    /// let mut engine = MyEngine::default();
    /// engine.route("/", Method::GET, Box::new(|_, _| {}));
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>);

    /// Adds a middleware to the application.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to add.
    fn use_middleware(&mut self, middleware: impl NgynMiddleware + 'static);
}
