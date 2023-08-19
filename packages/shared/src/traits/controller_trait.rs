/// `RustleControllerInit` is a trait that defines the basic structure of a controller initializer in Rustle.
pub trait RustleControllerInit: Send + Sync {
    /// Creates a new instance of the controller.
    /// This is for internal use only.
    fn new() -> Box<dyn RustleController>;
}

/// `RustleController` is a trait that defines the basic structure of a controller in Rustle.
/// It is designed to be thread-safe.
pub trait RustleController: Send + Sync {
    /// Returns a vector of routes associated with the controller.
    /// Each route is represented as a tuple of (method, path, handler).
    fn routes(&self) -> Vec<(&str, &str, Box<dyn Fn() + Send + Sync>)>;
}
