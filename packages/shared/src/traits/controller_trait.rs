/// `RustleControllerInit` is a trait that defines the basic structure of a controller initializer in Rustle.
pub trait RustleControllerInit: Send + Sync {
    /// Creates a new instance of the controller.
    /// This is for internal use only.
    fn new() -> Box<dyn RustleController>;
}

/// `RustleController` is a trait that defines the basic structure of a controller in Rustle.
/// It is designed to be thread-safe.
pub trait RustleController: Send + Sync {
    fn register(&mut self);
    /// Returns the name of the controller.
    ///
    fn name(&self) -> &str;

    /// Add route to the controller.
    /// This is for internal use only.
    fn add_route(
        &mut self,
        path: String,
        http_method: String,
        handler: Box<
            dyn Fn(crate::RustleRequest, crate::RustleResponse) -> crate::RustleResponse
                + Send
                + Sync,
        >,
    );

    /// Returns a vector of routes for the controller.
    fn routes(
        &self,
    ) -> Vec<(
        String,
        String,
        &Box<
            dyn Fn(crate::RustleRequest, crate::RustleResponse) -> crate::RustleResponse
                + Send
                + Sync,
        >,
    )>;
}
