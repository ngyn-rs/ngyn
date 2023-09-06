/// `RustleControllerInit` is a trait that defines the basic structure of a controller initializer in Rustle.
pub trait RustleControllerInit: Send + Sync {
    /// Creates a new instance of the controller.
    /// This is for internal use only.
    fn new() -> Box<dyn RustleController>;
}

/// `RustleController` is a trait that defines the basic structure of a controller in Rustle.
/// It is designed to be thread-safe.
pub trait RustleController: Send + Sync {
    /// Returns the name of the controller.
    ///
    fn name(&self) -> &str;

    /// Add route to the controller.
    /// This is for internal use only.
    fn add_route(&mut self, path: String, http_method: String, handler: String);

    /// Returns a vector of routes for the controller.
    fn routes(&self) -> Vec<(String, String, String)>;

    /// Returns a `RustleResponse` for the controller.
    /// This is for internal use only.
    fn handle(
        &self,
        handler: String,
        req: crate::RustleRequest,
        res: crate::RustleResponse,
    ) -> crate::RustleResponse;
}
