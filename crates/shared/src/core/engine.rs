use super::Handler;
use crate::Method;

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
    /// ```
    /// use crate::{Method, NgynEngine};
    ///
    /// struct MyEngine;
    ///
    /// let mut engine = MyEngine::default();
    /// engine.route("/", Method::GET, Box::new(|_, _| {}));
    /// ```
    fn route(&mut self, path: &str, method: Method, handler: Box<Handler>);
}
